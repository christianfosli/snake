namespace HighScoreApi.Common

open System

module Types =
    // --- Simple types --- //
    type UserName = string
    type Score = private Score of int
    type ValidationError = ValidationError of string

    module Score =
        let max = 12 * 12 - 1

        let value (Score score) = score

        let create n =
            if n < 0 || n > max then
                Error(ValidationError "score out of range")
            else
                Ok(Score n)

    // --- Domain Models --- //
    type HighScore =
        { UserName: UserName
          Score: Score
          TimeStamp: DateTimeOffset }

module DbUtils =
    open Types
    open Dapper
    open Polly
    open Microsoft.Data.SqlClient

    let connString =
        Environment.GetEnvironmentVariable "CONNECTION_STRING"

    type SqlScoreHandler() =
        inherit SqlMapper.TypeHandler<Score>()

        override __.SetValue(param, value) = param.Value <- value |> Score.value

        override __.Parse value =
            let value = value :?> int

            match value |> Score.create with
            | Ok score -> score
            | Error e -> failwithf "Error parsing to Score: %A" e

    let dbConnection connString =
        SqlMapper.AddTypeHandler(SqlScoreHandler())
        new SqlConnection(connString)

    let queryWithRetries<'entity> (connection: SqlConnection) (sql: string) =
        let retryPolicy =
            Policy
                .Handle<SqlException>()
                .WaitAndRetryAsync(
                    seq {
                        TimeSpan.FromSeconds 2.0
                        TimeSpan.FromSeconds 4.0
                        TimeSpan.FromSeconds 6.0
                    }
                )

        async {
            try
                let! res =
                    retryPolicy.ExecuteAsync(fun () -> connection.QueryAsync<'entity>(sql))
                    |> Async.AwaitTask

                return Ok res
            with
            | ex -> return Error ex
        }

module WebUtils =
    open System.Net
    open Microsoft.Azure.Functions.Worker.Http

    /// Create response with CORS header set to allow all origins.
    /// Work-around for bad CORS support in Azure functions docker containers
    let resWithOkCors (status: HttpStatusCode) (req: HttpRequestData) =
        let res = req.CreateResponse(status)
        res.Headers.Add("Access-Control-Allow-Origin", "*")
        res

    let okResWithOkCors (req: HttpRequestData) = resWithOkCors HttpStatusCode.OK req

    let badReqWithOkCors (req: HttpRequestData) =
        resWithOkCors HttpStatusCode.BadRequest req
