namespace HighScoreApi.Common

open Types

module Dto =
    type HighScoreDto = { UserName: string; Score: int }

    module HighScoreDto =
        open System

        let toHighScore dto =
            match Score.create dto.Score with
            | Ok score ->
                Ok
                    { UserName = dto.UserName
                      Score = score
                      TimeStamp = DateTimeOffset.UtcNow }
            | Error e -> Error e

        let fromHighScore (highscore: HighScore) =
            { UserName = highscore.UserName
              Score = Score.value highscore.Score }
