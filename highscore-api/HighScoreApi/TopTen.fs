namespace HighScoreApi

open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Data.SqlClient
open Microsoft.Extensions.Logging

open Common.DbUtils
open Common.WebUtils
open Common.Dto.HighScoreDto
open Common.Types

module TopTen =

    let topScores (connection: SqlConnection) =
        async {
            let! result =
                queryWithRetries<HighScore>
                    connection
                    "select top(10) [UserName],[Score],[TimeStamp]
                     from [highscores]
                     order by [Score] desc, [TimeStamp] asc"

            do! connection.CloseAsync() |> Async.AwaitTask

            return result
        }

    [<Function("TopTen")>]
    let run
        ([<HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)>] req: HttpRequestData)
        (ctx: FunctionContext)
        =
        let log = ctx.GetLogger()

        async {
            let! topScores = connString |> dbConnection |> topScores

            return
                match topScores with
                | Ok scores ->
                    Seq.length scores
                    |> sprintf "%d scores retrieved successfully"
                    |> log.LogInformation

                    let res = okResWithOkCors req

                    res
                        .WriteAsJsonAsync(Seq.map fromHighScore scores)
                        .AsTask()
                    |> Async.AwaitTask
                    |> Async.RunSynchronously

                    res

                | Error e ->
                    sprintf "Failed to get top ten: %A" e
                    |> log.LogError

                    failwith "An error occured"

        }
        |> Async.StartAsTask
