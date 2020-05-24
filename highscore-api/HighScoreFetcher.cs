using System;
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

            var topTen = await conn.QueryAsync<HighScore>(@"select top(10) * from highscores order by [score] desc");

            // It is not yet possible to configure CORS for az functions when
            // running locally inside a container
            // issue: https://github.com/Azure/azure-functions-host/issues/5090
            // so we'll adjust the headers ourselves:
            req.HttpContext.Response.Headers.Add("Access-Control-Allow-Origin", "*");

            return new OkObjectResult(topTen.ToList());
        }
    }
}
