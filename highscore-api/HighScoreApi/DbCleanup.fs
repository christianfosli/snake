namespace HighScoreApi

open System
open Dapper
open FSharp.Data.LiteralProviders
open Microsoft.Azure.Functions.Worker
open Microsoft.Data.SqlClient
open Microsoft.Extensions.Logging

open Common.DbUtils

module DbCleanup =
    // Run every sunday unless otherwise specified AT COMPILE TIME
    [<Literal>]
    let Schedule =
        Env<"CRON_CLEANUP_SCHEDULE", "0 0 0 * * SUN">
            .Value

    let removeNonTopHighScores connString =
        // TODO
        0

    [<Function("CleanupJob")>]
    let run ([<TimerTrigger(Schedule)>] myTimer: TimerInfo, ctx: FunctionContext) =
        let log = ctx.GetLogger()

        sprintf "Database clean-up triggered at: %A" DateTime.Now
        |> log.LogInformation

//removeNonTopHighScores connString
//|> sprintf "%d rows deleted"
//|> log.LogInformation
