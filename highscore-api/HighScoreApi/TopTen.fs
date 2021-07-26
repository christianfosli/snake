namespace HighScoreApi

open System.Net
open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Extensions.Logging
open MongoDB.Driver

open HighScoreApi.Common
open HighScoreApi.Common.Dto

module TopTen =

    let topten (collection: IMongoCollection<HighScoreDocument>) =
        let sortByScore =
            Builders<HighScoreDocument>
                .Sort.Descending(fun s -> s.Score :> obj)
                .Ascending(fun s -> s.TimeStamp :> obj)

        async {
            try
                let! topten =
                    collection
                        .Find(fun _ -> true)
                        .Sort(sortByScore)
                        .Limit(10)
                        .ToListAsync()
                    |> Async.AwaitTask

                return
                    topten
                    |> Seq.map HighScoreDocument.toHighScore
                    |> Seq.map HighScoreDto.fromHighScore
                    |> Ok
            with
            | ex -> return Error ex

        }


    [<Function("TopTen")>]
    let run
        ([<HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)>] req: HttpRequestData)
        (ctx: FunctionContext)
        =
        let log = ctx.GetLogger()

        async {
            let! topScores = topten DbUtils.highscores

            return
                match topScores with
                | Ok scores ->
                    Seq.length scores
                    |> sprintf "%d scores retrieved successfully"
                    |> log.LogInformation

                    let res = WebUtils.okResWithOkCors req

                    res.WriteAsJsonAsync(scores).AsTask()
                    |> Async.AwaitTask
                    |> Async.RunSynchronously

                    res

                | Error e ->
                    sprintf "Failed to get top ten: %A" e
                    |> log.LogError

                    let res =
                        WebUtils.resWithOkCors HttpStatusCode.InternalServerError req

                    res.WriteString "An error occured trying to fetch topten. Details in server logs."
                    res
        }
        |> Async.StartAsTask
