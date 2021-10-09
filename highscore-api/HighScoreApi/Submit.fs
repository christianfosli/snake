namespace HighScoreApi

open System.IO
open System.Net
open System.Text.Json
open System.Text.Json.Serialization
open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Azure.WebJobs.Extensions.OpenApi.Core.Attributes
open Microsoft.Extensions.Logging
open MongoDB.Driver

open HighScoreApi.Common
open HighScoreApi.Common.Dto

module Submit =

    let deserialize (body: string) =
        let options = JsonSerializerOptions()
        options.Converters.Add(JsonFSharpConverter())
        options.PropertyNameCaseInsensitive <- true

        try
            JsonSerializer.Deserialize<HighScoreDto>(body, options)
            |> Ok
        with
        | err -> Error err.Message

    type PersistResult =
        | Created
        | AlreadyExists
        | Err of string

    let persist (collection: IMongoCollection<HighScoreDocument>) highscore =
        async {
            try
                let! existingScore =
                    collection
                        .Find(fun s -> s.Id = highscore.Id)
                        .FirstOrDefaultAsync()
                    |> Async.AwaitTask

                if box existingScore = null then
                    do!
                        collection.InsertOneAsync(highscore)
                        |> Async.AwaitTask

                    return PersistResult.Created
                else
                    return PersistResult.AlreadyExists
            with
            | err -> return PersistResult.Err err.Message
        }

    [<OpenApiOperation("submit", [| "highscores" |], Summary = "Submit new highscore")>]
    [<OpenApiRequestBody("application/json", typeof<HighScoreDto>)>]
    [<OpenApiResponseWithBody(HttpStatusCode.Created,
                              "text/plain",
                              typeof<string>,
                              Description = "OK - New highscore created")>]
    [<OpenApiResponseWithBody(HttpStatusCode.OK,
                              "text/plain",
                              typeof<string>,
                              Description = "OK - Highscore has been submitted already")>]
    [<OpenApiResponseWithBody(HttpStatusCode.BadRequest,
                              "text/plain",
                              typeof<string>,
                              Description = "If highscore fails deserialization")>]
    [<OpenApiResponseWithBody(HttpStatusCode.UnprocessableEntity,
                              "text/plain",
                              typeof<string>,
                              Description = "If highscore fails validation")>]
    [<Function("Submit")>]
    let run
        ([<HttpTrigger(AuthorizationLevel.Anonymous, "post", "options", Route = null)>] req: HttpRequestData)
        (ctx: FunctionContext)
        =
        let log = ctx.GetLogger()

        async {
            use stream = new StreamReader(req.Body)

            let! body = stream.ReadToEndAsync() |> Async.AwaitTask

            match deserialize body with
            | Ok highscore ->
                match highscore |> HighScoreDto.toHighScore with
                | Ok highscore ->
                    log.LogInformation $"persisting %A{highscore}"

                    let! dbRes = persist DbUtils.highscores (HighScoreDocument.fromHighScore highscore)

                    let res =
                        match dbRes with
                        | PersistResult.Created -> req.CreateResponse HttpStatusCode.Created
                        | PersistResult.AlreadyExists -> req.CreateResponse HttpStatusCode.OK
                        | PersistResult.Err e -> req.CreateResponse HttpStatusCode.InternalServerError

                    log.LogInformation $"%A{dbRes}"
                    res.WriteString $"%A{dbRes}"
                    return res

                | Error e ->
                    sprintf "validation error: %A" e |> log.LogError

                    let res =
                        req.CreateResponse HttpStatusCode.UnprocessableEntity

                    res.WriteString $"%A{e}"
                    return res
            | Error e ->
                sprintf "deserialize error: %A" e |> log.LogError

                let res =
                    req.CreateResponse HttpStatusCode.BadRequest

                res.WriteString $"%A{e}"
                return res
        }
        |> Async.StartAsTask
