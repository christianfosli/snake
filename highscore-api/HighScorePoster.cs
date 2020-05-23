using System;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using Dapper;
using Microsoft.Data.SqlClient;

namespace highscore_api
{
    public static class HighScorePoster
    {
        [FunctionName("HighScorePoster")]
        public static async Task<IActionResult> Run(
            [HttpTrigger(AuthorizationLevel.Anonymous, "post", Route = null)] HttpRequest req,
            ILogger log)
        {
            log.LogInformation("HighScorePoster triggered");

            var body = await new StreamReader(req.Body).ReadToEndAsync();
            var highscore = JsonSerializer.Deserialize<HighScore>(body, new JsonSerializerOptions {
                PropertyNameCaseInsensitive = true,
            });

            var connectionString = Environment.GetEnvironmentVariable("CONNECTION_STRING")
                ?? throw new InvalidOperationException("No DB connection string found");

            using var conn = new SqlConnection(connectionString);

            await conn.ExecuteAsync("insert into highscores(UserName, Score, TimeStamp) " +
                "values (@UserName, @Score, @TimeStamp)", highscore);

            // It is not yet possible to configure CORS for az functions when
            // running locally inside a container
            // issue: https://github.com/Azure/azure-functions-host/issues/5090
            // so we'll adjust the headers ourselves:
            req.HttpContext.Response.Headers.Add("Access-Control-Allow-Origin", "*");

            return new CreatedResult(nameof(HighScoreFetcher), highscore);
        }
    }
}
