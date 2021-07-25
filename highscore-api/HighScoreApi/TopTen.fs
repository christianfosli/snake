namespace HighScoreApi

open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Extensions.Logging
open MongoDB.Driver

open System.Linq
open System.Net

open HighScoreApi.Common
open Common.Dto
open Common.Types

module TopTen =

    let topten (collection: IMongoCollection<HighScoreDocument>) =
        let sortByScore =
            Builders<HighScoreDocument>
                .Sort.Descending(fun s -> s.Score :> obj)
                .Ascending(fun s -> s.TimeStamp :> obj)

        async {
            try
                let! sortedScores =
                    collection
                        .Find(fun _ -> true)
                        .Sort(sortByScore)
                        .ToListAsync()
                    |> Async.AwaitTask

                return
                    sortedScores
                    |> Seq.truncate 10
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

                    res.WriteString "An error occured"
                    res
        }
        |> Async.StartAsTask
