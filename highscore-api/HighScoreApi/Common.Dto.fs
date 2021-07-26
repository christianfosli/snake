namespace HighScoreApi.Common

open System
open MongoDB.Bson.Serialization.Attributes

open HighScoreApi.Common.Types

module Dto =
    /// HighScore type for serializing to JSON and sending to/from API
    type HighScoreDto = { UserName: string; Score: int }

    module HighScoreDto =

        let toHighScore dto : Result<HighScore, ValidationError> =
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

    /// HighScore type for serializing to BSON and sending to/from MongoDB
    [<CLIMutable>]
    type HighScoreDocument =
        { [<BsonId>]
          Id: string
          UserName: string
          Score: int
          TimeStamp: DateTimeOffset }

    module HighScoreDocument =

        let toHighScore (document: HighScoreDocument) : HighScore =
            { UserName = document.UserName
              Score =
                  match Score.create document.Score with
                  | Ok s -> s
                  | Error e -> failwithf "Bad data! Can't convert document %A due to: %A" document.Id e
              TimeStamp = document.TimeStamp }

        let fromHighScore (highscore: HighScore) : HighScoreDocument =
            { Id = HighScore.Uid highscore
              UserName = highscore.UserName
              Score = Score.value highscore.Score
              TimeStamp = highscore.TimeStamp }
