using System;
using System.Collections.Generic;
using System.Text;

namespace highscore_api
{
    public class HighScore
    {
        public HighScore()
        {
            TimeStamp = DateTimeOffset.UtcNow;
        }

        public string UserName { get; set; }
        public int Score { get; set; }
        public DateTimeOffset TimeStamp { get; private set; }
    }
}
