namespace HighScoreApi.Common

open MongoDB.Driver
open System

open HighScoreApi.Common.Dto

module DbUtils =

    let connString =
        Environment.GetEnvironmentVariable "CONNECTION_STRING"

    let highscores: IMongoCollection<HighScoreDocument> =
        let client = MongoClient(connString)
        let database = client.GetDatabase("snake-highscore")
        database.GetCollection<HighScoreDocument>("highscore")
