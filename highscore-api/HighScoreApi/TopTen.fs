namespace HighScoreApi

open System
open System.Net
open Microsoft.Azure.Functions.Worker
open Microsoft.Azure.Functions.Worker.Http
open Microsoft.Extensions.Logging
open MongoDB.Driver

open HighScoreApi.Common
open HighScoreApi.Common.Dto

module TopTen =

    let topten (collection: IMongoCollection<HighScoreDocument>) (since: Option<DateTimeOffset>) =
        let dateFilter =
            match since with
            | Some dte -> Builders<HighScoreDocument>.Filter.Gt ((fun s -> s.TimeStamp), dte)
            | None -> Builders<HighScoreDocument>.Filter.Empty

        let sortByScore =
            Builders<HighScoreDocument>
                .Sort.Descending(fun s -> s.Score :> obj)
                .Ascending(fun s -> s.TimeStamp :> obj)

        async {
            try
                let! topten =
                    collection
                        .Find(dateFilter)
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

    let dateFromQueryParam (ctx: FunctionContext) (paramName: string) : Result<Option<DateTimeOffset>, string> =
        if ctx.BindingContext.BindingData.ContainsKey(paramName) then
            let dateStr =
                ctx.BindingContext.BindingData.[paramName] :?> string

            let couldParse, parsedDate = DateTimeOffset.TryParse dateStr

            if couldParse then
                Some parsedDate |> Ok
            else
                Error $"Unable to parse %s{dateStr} as a date"
        else
            Ok None


    [<Function("TopTen")>]
    let run ([<HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)>] req: HttpRequestData) (ctx: FunctionContext) =
        let log = ctx.GetLogger()

        async {
            match dateFromQueryParam ctx "since" with
            | Ok since ->
                log.LogInformation $"Finding topten since %A{since}"

                let! topScores = topten DbUtils.highscores since

                match topScores with
                | Ok scores ->
                    log.LogInformation $"%d{Seq.length scores} scores found"
                    let res = req.CreateResponse HttpStatusCode.OK

                    do!
                        res.WriteAsJsonAsync(scores).AsTask()
                        |> Async.AwaitTask

                    return res

                | Error e ->
                    log.LogError $"Failed to get top ten due to %A{e}"

                    let res =
                        req.CreateResponse HttpStatusCode.InternalServerError

                    do!
                        res.WriteStringAsync "Server error trying to fetch topten from database."
                        |> Async.AwaitTask

                    return res

            | Error parseError ->
                let res =
                    req.CreateResponse HttpStatusCode.BadRequest

                do! res.WriteStringAsync parseError |> Async.AwaitTask
                return res
        }
        |> Async.StartAsTask
