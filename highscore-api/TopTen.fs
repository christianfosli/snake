namespace HighScoreApi

open Dapper;
open Microsoft.AspNetCore.Mvc
open Microsoft.Azure.WebJobs
open Microsoft.Azure.WebJobs.Extensions.Http
open Microsoft.AspNetCore.Http
open Microsoft.Data.SqlClient
open Microsoft.Extensions.Logging
open Microsoft.Extensions.Primitives

open Common

module TopTen =

    let topScores connString = async {
        use conn = new SqlConnection(connString)
        let! result =
            conn.QueryAsync<HighScoreDto>
                "select top(10) [UserName],[Score]
                 from [highscores]
                 order by [Score] desc, [TimeStamp] asc"
            |> Async.AwaitTask
        return result
    }

    [<FunctionName("TopTen")>]
    let run ([<HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)>]req: HttpRequest) (log: ILogger) =
        async {
            log.LogInformation "TopTen triggered"

            // It is not yet possible to configure CORS for az functions locally in containers
            // so we just fix headers (ref https://github.com/Azure/azure-functions-host/issues/5090)
            req.HttpContext.Response.Headers.Add ("Access-Control-Allow-Origin", new StringValues("*"))

            let! response = topScores connString
            return OkObjectResult response :> IActionResult
        } |> Async.StartAsTask