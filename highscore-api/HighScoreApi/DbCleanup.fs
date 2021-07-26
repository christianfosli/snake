namespace HighScoreApi

open System
open FSharp.Data.LiteralProviders
open Microsoft.Azure.Functions.Worker
open Microsoft.Extensions.Logging
open MongoDB.Driver

open HighScoreApi.Common
open HighScoreApi.Common.Dto

module DbCleanup =
    // Run every sunday unless otherwise specified AT COMPILE TIME
    [<Literal>]
    let Schedule =
        Env<"CRON_CLEANUP_SCHEDULE", "0 0 0 * * SUN">
            .Value

    let removeNonTopHighScores (collection: IMongoCollection<HighScoreDocument>) =
        let sortByScore =
            Builders<HighScoreDocument>
                .Sort.Descending(fun s -> s.Score :> obj)
                .Ascending(fun s -> s.TimeStamp :> obj)

        let toDelete = collection.Find(fun _ -> true).Sort(sortByScore).Skip(15).ToList() |> Seq.map (fun x -> x.Id)
        let deleteFilter = Builders<HighScoreDocument>.Filter.In((fun x -> x.Id), toDelete)

        let result = collection.DeleteMany(deleteFilter)
        result.DeletedCount

    [<Function("CleanupJob")>]
    let run ([<TimerTrigger(Schedule)>] myTimer: TimerInfo, ctx: FunctionContext) =
        let log = ctx.GetLogger()

        sprintf "Database clean-up triggered at: %A" DateTime.Now
        |> log.LogInformation

        removeNonTopHighScores DbUtils.highscores
        |> sprintf "%d rows deleted"
        |> log.LogInformation
