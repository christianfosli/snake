namespace HighScoreApi

open System.Net
open System.IO
open System.Text.Json
open System.Text.Json.Serialization
open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Extensions.Logging
open MongoDB.Driver

open HighScoreApi.Common
open Common.Types
open Common.Dto

module Submit =

    let deserialize (body: string) =
        let options = JsonSerializerOptions()
        options.Converters.Add(JsonFSharpConverter())
        options.PropertyNameCaseInsensitive <- true
        JsonSerializer.Deserialize<HighScoreDto>(body, options)

    let persist (collection: IMongoCollection<HighScoreDocument>) (log: ILogger) highscore =
        async {
            let! existingScore =
                collection
                    .Find(fun s -> s.Id = highscore.Id)
                    .FirstOrDefaultAsync()
                |> Async.AwaitTask

            if box existingScore = null then
                do!
                    collection.InsertOneAsync(highscore)
                    |> Async.AwaitTask
            else
                sprintf "HighScore already exists with id %A. Nothing to do" highscore.Id
                |> log.LogInformation
        }

    [<Function("Submit")>]
    let run
        ([<HttpTrigger(AuthorizationLevel.Anonymous, "post", "options", Route = null)>] req: HttpRequestData)
        (ctx: FunctionContext)
        =
        let log = ctx.GetLogger()

        async {
            sprintf "Submit triggered with method %A" req.Method
            |> log.LogInformation

            if req.Method.ToLower() = "options" then
                return WebUtils.okResWithOkCors req
            else
                use stream = new StreamReader(req.Body)

                let! body = stream.ReadToEndAsync() |> Async.AwaitTask

                match deserialize body |> HighScoreDto.toHighScore with
                | Ok highscore ->
                    sprintf "persisting %A" highscore
                    |> log.LogInformation

                    do! persist DbUtils.highscores log (HighScoreDocument.fromHighScore highscore)

                    return WebUtils.okResWithOkCors req
                | Error e ->
                    sprintf "%A" e |> log.LogError

                    let res = WebUtils.badReqWithOkCors req
                    res.WriteString $"%A{e}"
                    return res
        }
        |> Async.StartAsTask
