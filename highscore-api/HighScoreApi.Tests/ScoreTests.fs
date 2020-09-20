namespace HighScoreApi.Tests

open Xunit
open FsUnit.Xunit
open HighScoreApi.Common.Types

module ScoreTests =

    type TestResult =
        | Okay
        | SomeError

    [<Fact>]
    let ``Given a positive number less than max then create should succeed`` () =
        let result =
            match Score.create 50 with
            | Ok _ -> Okay
            | Error _ -> SomeError

        result |> should equal Okay

    [<Fact>]
    let ``Given a negative number then create should fail`` () =
        let result =
            match Score.create -5 with
            | Ok _ -> Okay
            | Error _ -> SomeError

        result |> should equal SomeError

    [<Fact>]
    let ``Given a number higher than max then create should fail`` () =
        let result =
            match Score.max + 5 |> Score.create with
            | Ok _ -> Okay
            | Error _ -> SomeError

        result |> should equal SomeError
