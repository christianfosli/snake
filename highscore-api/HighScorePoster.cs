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
    public static class HighScorePoster
    {
        [FunctionName("HighScorePoster")]
        public static async Task<IActionResult> Run(
            [HttpTrigger(AuthorizationLevel.Anonymous, "post", Route = null)] HttpRequest req,
            ILogger log)
        {
            log.LogInformation("HighScorePoster triggered");

            var body = await new StreamReader(req.Body).ReadToEndAsync();
            var highscore = JsonConvert.DeserializeObject<HighScore>(body);

            // TODO: Store score in a database

            return new CreatedResult(nameof(HighScoreFetcher), highscore);
        }
    }
}
