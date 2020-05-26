namespace Company.Function

open System
open FSharp.Data.Sql
open FSharp.Data.LiteralProviders
open Microsoft.Azure.WebJobs
open Microsoft.Extensions.Logging

module DbCleanup =
    let [<Literal>] dbVendor = Common.DatabaseProviderTypes.MSSQLSERVER
    let [<Literal>] connString = Env<"CONNECTION_STRING", "Server=tcp:localhost,1433;Database=snake-hs;User ID=sa;Password=Secret_01">.Value

    type sql = SqlDataProvider<dbVendor, connString, UseOptionTypes=true>

    [<FunctionName("TimerTriggerCSharp")>]
    let run([<TimerTrigger("0 * * * * *")>]myTimer: TimerInfo, log: ILogger) =
        let msg = sprintf "Database CleanUp function triggered at: %A" DateTime.Now
        log.LogInformation msg

        let ctx = sql.GetDataContext()
        ctx.``Design Time Commands``.SaveContextSchema

        query {
            for hs in ctx.Dbo.Highscores do
            sortByDescending (hs.Score, hs.TimeStamp)
            skip 15
            select (hs)
        } |> Seq.``delete all items from single table`` |> Async.RunSynchronously

