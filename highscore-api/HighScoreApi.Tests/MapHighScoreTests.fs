namespace HighScoreApi.Tests

open System
open HighScoreApi.Common.Dto
open Xunit
open FsUnit.Xunit

module MapHighScoreTests =

    module ``Given highscore dto`` =
        let dto = { UserName = "James"; Score = 5 }

        [<Fact>]
        let ``when mapping to highscore and document then document should contain correct fields`` () =
            let hs =
                match (HighScoreDto.toHighScore dto) with
                | Ok hs -> hs
                | Error e -> failwith $"Error mapping dto -> highscore: %A{e}"

            let document = HighScoreDocument.fromHighScore hs

            document.UserName |> should equal dto.UserName
            document.Score |> should equal dto.Score

    module ``Given highscore document`` =
        let document =
            { Id = "20210101-123456-s50"
              UserName = "James"
              Score = 43
              TimeStamp = DateTimeOffset.Parse("2021-01-01") }

        [<Fact>]
        let ``when mapping to highscore and dto then dto should contain correct fields`` () =
            let hs = HighScoreDocument.toHighScore document
            let dto = HighScoreDto.fromHighScore hs

            dto.UserName |> should equal document.UserName
            dto.Score |> should equal document.Score
