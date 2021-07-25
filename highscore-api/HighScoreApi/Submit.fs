namespace HighScoreApi

open Dapper
open System.Net
open System.IO
open System.Text.Json
open System.Text.Json.Serialization
open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Data.SqlClient
open Microsoft.Extensions.Logging
open Microsoft.Extensions.Primitives

open Common.DbUtils
open Common.Dto
open Common.Dto.HighScoreDto

module Submit =

    let deserialize (body: string) =
        let options = JsonSerializerOptions()
        options.Converters.Add(JsonFSharpConverter())
        options.PropertyNameCaseInsensitive <- true
        JsonSerializer.Deserialize<HighScoreDto>(body, options)

    let persist (connection: SqlConnection) highscore =
        async {
            let! rows =
                connection.ExecuteAsync(
                    "insert into [highscores]([UserName], [Score], [TimeStamp])
                 values (@UserName, @Score, @TimeStamp)",
                    highscore
                )
                |> Async.AwaitTask

            do! connection.CloseAsync() |> Async.AwaitTask

            return rows
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

            use stream = new StreamReader(req.Body)
            let! body = stream.ReadToEndAsync() |> Async.AwaitTask

            match deserialize body |> toHighScore with
            | Ok highscore ->
                sprintf "persisting %A" highscore
                |> log.LogInformation

                let! rows = persist (connString |> dbConnection) highscore

                sprintf "%d rows affected" rows
                |> log.LogInformation

                return req.CreateResponse(HttpStatusCode.OK)
            | Error e ->
                sprintf "%A" e |> log.LogError

                let res =
                    req.CreateResponse(HttpStatusCode.BadRequest)

                sprintf "An error occured: %A" e
                |> res.WriteString

                return res
        }
        |> Async.StartAsTask
