using System;
using System.Data;
using System.IO;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using Dapper;
using System.Linq;
using Microsoft.Data.SqlClient;

namespace highscore_api
{
    public static class HighScoreFetcher
    {
        [FunctionName("HighScoreFetcher")]
        public static async Task<IActionResult> Run(
            [HttpTrigger(AuthorizationLevel.Anonymous, "get", Route = null)] HttpRequest req,
            ILogger log)
        {
            log.LogInformation("HighScoreFetcher triggered");

            var connectionString = Environment.GetEnvironmentVariable("CONNECTION_STRING")
                ?? throw new InvalidOperationException("No DB connection string found");

            using var conn = new SqlConnection(connectionString);

            var highscores = await conn.QueryAsync<HighScore>(@"select * from highscores");

            return new OkObjectResult(highscores.ToList());
        }
    }
}
