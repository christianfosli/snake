namespace HighScoreApi

open Dapper
open System.IO
open System.Text.Json
open System.Text.Json.Serialization
open Microsoft.AspNetCore.Mvc
open Microsoft.Azure.WebJobs
open Microsoft.Azure.WebJobs.Extensions.Http
open Microsoft.AspNetCore.Http
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
                connection.ExecuteAsync
                    ("insert into [highscores]([UserName], [Score], [TimeStamp])
                 values (@UserName, @Score, @TimeStamp)",
                     highscore)
                |> Async.AwaitTask

            do! connection.CloseAsync() |> Async.AwaitTask

            return rows
        }

    [<FunctionName("Submit")>]
    let run ([<HttpTrigger(AuthorizationLevel.Anonymous, "post", "options", Route = null)>] req: HttpRequest)
            (log: ILogger)
            =
        async {
            sprintf "Submit triggered with method %A" req.Method
            |> log.LogInformation

            // It is not yet possible to configure CORS for az functions locally in containers
            // so we just fix headers (ref https://github.com/Azure/azure-functions-host/issues/5090)
            req.HttpContext.Response.Headers.Add("Access-Control-Allow-Origin", StringValues "*")

            // We also need to handle CORS pre-flight requests (Would be automatic on Azure)
            if req.Method.ToLower() = "options" then
                return OkResult() :> IActionResult
            else
                use stream = new StreamReader(req.Body)
                let! body = stream.ReadToEndAsync() |> Async.AwaitTask

                match deserialize body |> toHighScore with
                | Ok highscore ->
                    sprintf "persisting %A" highscore
                    |> log.LogInformation

                    let! rows = persist (connString |> dbConnection) highscore

                    sprintf "%d rows affected" rows
                    |> log.LogInformation

                    return OkResult() :> IActionResult
                | Error e ->
                    sprintf "%A" e |> log.LogError
                    return BadRequestObjectResult(sprintf "An error occured: %A" e) :> IActionResult

        }
        |> Async.StartAsTask
