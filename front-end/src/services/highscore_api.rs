use chrono::{DateTime, Utc};
use js_sys::Error;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::RequestInit;
use web_sys::{Request, RequestMode, Response};

use crate::highscores::HighScore;

pub struct HighScoreApi {
    base_url: String,
}

#[derive(Serialize)]
struct QueryParams {
    since: DateTime<Utc>,
}

impl HighScoreApi {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn top_ten(&self, since: Option<DateTime<Utc>>) -> Result<Vec<HighScore>, JsValue> {
        let mut options = RequestInit::new();
        options.method("GET").mode(RequestMode::Cors);

        let request_url = if let Some(since) = since {
            let query_params = serde_qs::to_string(&QueryParams { since })
                .map_err(|e| Error::new(&format!("Serialize error: {:?}", e)))?;

            format!("{}/api/topten?{}", self.base_url, query_params)
        } else {
            format!("{}/api/topten", self.base_url)
        };

        let request = Request::new_with_str_and_init(&request_url, &options)?;
        request.headers().set("Accept", "application/json")?;

        let window = web_sys::window().ok_or_else(|| Error::new("Windows was none"))?;

        let res = JsFuture::from(window.fetch_with_request(&request)).await?;
        let res: Response = res.dyn_into()?;

        if res.ok() {
            let json = JsFuture::from(res.json()?).await?;

            let highscores: Vec<HighScore> = json
                .into_serde()
                .map_err(|e| Error::new(&format!("Serialization failed due to {}", e)))?;

            Ok(highscores)
        } else {
            let response_text = JsFuture::from(res.text()?)
                .await?
                .as_string()
                .ok_or_else(|| "".to_owned());

            Err(Error::new(&format!(
                "TopTen API call failed failed. Status {}, {:?}",
                res.status(),
                response_text
            ))
            .into())
        }
    }

    pub async fn submit(&self, highscore: &HighScore) -> Result<(), JsValue> {
        let json = serde_json::to_string(highscore)
            .map_err(|e| Error::new(&format!("Error during deserialization: {:?}", e)))?;

        let mut options = RequestInit::new();
        options
            .method("POST")
            .mode(RequestMode::Cors)
            .body(Some(&json.into()));

        let req_url = format!("{}/api/submit", self.base_url);
        let request = Request::new_with_str_and_init(&req_url, &options)?;

        request.headers().set("Accept", "application/json")?;
        request.headers().set("Content-Type", "application/json")?;

        let window = web_sys::window().ok_or_else(|| Error::new("Windows was none"))?;

        let res = JsFuture::from(window.fetch_with_request(&request)).await?;
        let res: Response = res.dyn_into()?;

        if res.ok() {
            Ok(())
        } else {
            let response_text = JsFuture::from(res.text()?)
                .await?
                .as_string()
                .ok_or_else(|| "".to_owned());

            Err(Error::new(&format!(
                "Submit API call failed. Status {}, {:?}",
                res.status(),
                response_text
            ))
            .into())
        }
    }
}
