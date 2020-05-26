namespace HighScoreApi

open System

module Common =

    let connString = Environment.GetEnvironmentVariable "CONNECTION_STRING"

    type HighScore = {
        UserName: string;
        Score: int;
        TimeStamp: DateTimeOffset;
    }

    type HighScoreDto = {
        UserName: string;
        Score: int;
    }

    let toDomain dto: HighScore =
        { UserName = dto.UserName; Score = dto.Score; TimeStamp = DateTimeOffset.UtcNow; }

