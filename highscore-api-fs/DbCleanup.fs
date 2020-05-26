namespace Company.Function

open System
open Dapper
open FSharp.Data.LiteralProviders
open Microsoft.Data.SqlClient
open Microsoft.Azure.WebJobs
open Microsoft.Extensions.Logging

module DbCleanup =
    // Run every sunday unless otherwise specified AT COMPILE TIME
    let [<Literal>] schedule = Env<"CRON_CLEANUP_SCHEDULE", "0 0 0 * * SUN">.Value

    let connString = Environment.GetEnvironmentVariable "CONNECTION_STRING"

    let removeNonTopHighScores connString =
        use conn = new SqlConnection(connString)
        conn.Execute
            "with ToDelete as (
                select * from highscores
                order by score desc
                offset 15 rows)
            delete from ToDelete;"

    [<FunctionName("CleanupHighScoresJob")>]
    let run([<TimerTrigger(schedule)>]myTimer: TimerInfo, log: ILogger) =
        sprintf "Database clean-up triggered at: %A" DateTime.Now
            |> log.LogInformation

        removeNonTopHighScores connString
            |> sprintf "%d rows deleted"
            |> log.LogInformation

