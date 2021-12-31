use reqwest::Client;
use serde::Serialize;
use time::OffsetDateTime;

use crate::highscores::HighScore;

pub struct HighScoreApi {
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct QueryParams {
    since: OffsetDateTime,
}

impl HighScoreApi {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub async fn top_ten(
        &self,
        since: Option<OffsetDateTime>,
    ) -> Result<Vec<HighScore>, anyhow::Error> {
        let request_url = if let Some(since) = since {
            let query_params = serde_qs::to_string(&QueryParams { since })?;
            format!("{}/api/topten?{}", self.base_url, query_params)
        } else {
            format!("{}/api/topten", self.base_url)
        };

        let res = self
            .client
            .get(&request_url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(res)
    }

    pub async fn submit(&self, highscore: &HighScore) -> Result<(), anyhow::Error> {
        self.client
            .post(&format!("{}/api/submit", self.base_url))
            .json(highscore)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
