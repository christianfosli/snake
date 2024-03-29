use highscore_types::HighScoreDto;
use reqwest::Client;
use serde::Serialize;
use time::OffsetDateTime;

pub struct HighScoreApi {
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct QueryParams {
    #[serde(with = "time::serde::rfc3339")]
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
    ) -> Result<Vec<HighScoreDto>, anyhow::Error> {
        let request_url = if let Some(since) = since {
            let query_params = serde_qs::to_string(&QueryParams { since })?;
            format!("{base}/topten?{query_params}", base = self.base_url)
        } else {
            format!("{base}/topten", base = self.base_url)
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

    pub async fn submit(&self, highscore: &HighScoreDto) -> Result<(), anyhow::Error> {
        self.client
            .post(&format!("{base}/submit", base = self.base_url))
            .json(highscore)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::format_description::well_known::Rfc3339;

    #[test]
    fn should_serialize_query_params_correctly() {
        let since = OffsetDateTime::parse("2021-01-01T00:00:00Z", &Rfc3339).unwrap();
        let query_params = serde_qs::to_string(&QueryParams { since }).unwrap();
        assert_eq!("since=2021-01-01T00%3A00%3A00Z", &query_params);
    }
}
