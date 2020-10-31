namespace HighScoreApi

open Dapper
open Microsoft.AspNetCore.Mvc
open Microsoft.Azure.WebJobs
open Microsoft.Azure.WebJobs.Extensions.Http
open Microsoft.AspNetCore.Http
open Microsoft.Data.SqlClient
open Microsoft.Extensions.Logging
open Microsoft.Extensions.Primitives

open Common.Types
open Common.DbUtils
open Common.Dto.HighScoreDto

module TopTen =

    let topScores (connection: SqlConnection) =
        async {
            let! result =
                connection.QueryAsync<HighScore> "select top(10) [UserName],[Score],[TimeStamp]
                 from [highscores]
                 order by [Score] desc, [TimeStamp] asc"
                |> Async.AwaitTask

            do! connection.CloseAsync() |> Async.AwaitTask

            return result
        }

    [<FunctionName("TopTen")>]
    let run ([<HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)>] req: HttpRequest) (log: ILogger) =
        async {
            log.LogInformation "TopTen triggered"

            // It is not yet possible to configure CORS for az functions locally in containers
            // so we just fix headers (ref https://github.com/Azure/azure-functions-host/issues/5090)
            req.HttpContext.Response.Headers.Add("Access-Control-Allow-Origin", StringValues "*")

            let! topScores = connString |> dbConnection |> topScores

            return OkObjectResult(topScores |> Seq.map fromHighScore) :> IActionResult
        }
        |> Async.StartAsTask