using System;
using System.IO;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Azure.WebJobs;
using Microsoft.Azure.WebJobs.Extensions.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Logging;
using Newtonsoft.Json;

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

            // TODO: fetch highscores from a database
            var highscores = new []
            {
                new HighScore { UserName = "ferris the crab", Score = 5},
                new HighScore { UserName = "snoop the snake", Score = 6},
            };

            return new OkObjectResult(highscores);
        }
    }
}
