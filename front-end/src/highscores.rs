use js_sys::Error;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode, Response};

const BASE_URL: Option<&'static str> = option_env!("HIGHSCORE_API_BASE_URL");

#[derive(Debug, Serialize, Deserialize)]
pub struct HighScore {
    #[serde(rename = "userName")]
    user_name: String,
    score: usize,
}

impl HighScore {
    fn to_table_row(&self) -> String {
        format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            self.user_name, self.score
        )
    }
}

pub async fn fetch_and_set_highscores() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Window was none")?;

    let tbody = window
        .document()
        .map(|doc| doc.query_selector("#highscore-tbody"))
        .ok_or_else(|| Error::new("Cant find highscore table"))??
        .map(|table| table.dyn_into::<HtmlElement>())
        .ok_or_else(|| Error::new("Highscore table was not a HtmlElement???"))??;

    let html = match fetch_highscores().await {
        Ok(highscores) => highscores
            .iter()
            .map(|h| h.to_table_row())
            .collect::<String>(),
        Err(e) => {
            log::error!("Failed to fetch highscores due to {:?}", e);
            String::from("<tr><td colspan=\"2\">Failed to fetch :-(</td></tr>")
        }
    };

    tbody.set_inner_html(&html);

    Ok(())
}

pub async fn fetch_highscores() -> Result<Vec<HighScore>, JsValue> {
    let mut options = RequestInit::new();
    options.method("GET").mode(RequestMode::Cors);

    let base_url = BASE_URL.ok_or_else(|| Error::new("Baseurl is undefined"))?;
    log::debug!("Fetching highscores with api url {}", base_url);

    let endpoint = format!("{}/api/topten", base_url);
    let request = Request::new_with_str_and_init(&endpoint, &options)?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().ok_or_else(|| Error::new("Windows was none"))?;

    let res: Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()?;

    let json = JsFuture::from(res.json()?).await?;

    let highscores: Vec<HighScore> = json
        .into_serde()
        .map_err(|e| Error::new(&format!("Serialization failed due to {}", e)))?;

    Ok(highscores)
}

pub async fn check_and_submit_highscore(score: usize) -> Result<(), JsValue> {
    let top_scores = fetch_highscores().await?;
    if top_scores.len() < 10 || top_scores.iter().any(|hs| hs.score < score) {
        log::debug!("Score {} is a highscore!", score);

        let window = web_sys::window().ok_or_else(|| Error::new("Window was none"))?;
        let user_name =
            match window.prompt_with_message("Please enter your name for the highscore table")? {
                Some(v) => v,
                None => {
                    log::warn!("highscore submission aborted as no username given");
                    return Ok(());
                }
            };

        let json = serde_json::to_string(&HighScore { user_name, score })
            .map_err(|e| Error::new(&format!("Error during deserialization: {:?}", e)))?;

        let mut options = RequestInit::new();
        options
            .method("POST")
            .mode(RequestMode::Cors)
            .body(Some(&json.into()));

        let request =
            Request::new_with_str_and_init(&format!("{}/api/submit", BASE_URL.unwrap()), &options)?;

        request.headers().set("Accept", "application/json")?;
        request.headers().set("Content-Type", "text/plain")?;

        let res: Response = JsFuture::from(window.fetch_with_request(&request))
            .await?
            .dyn_into()?;

        match res.ok() {
            true => {
                log::info!("Highscore submitted successfully");
                fetch_and_set_highscores().await?;
            }
            false => {
                log::error!("failed to submit highscore");
                return Err(Error::new("Failed to submit highscore").into());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_table_row_should_include_username() {
        let highscore = HighScore {
            user_name: String::from("testuser"),
            score: 0,
        };

        assert!(highscore
            .to_table_row()
            .find(&highscore.user_name)
            .is_some());
    }

    #[test]
    fn to_table_row_should_include_score() {
        let highscore = HighScore {
            user_name: String::from("testuser"),
            score: 5,
        };

        assert!(highscore
            .to_table_row()
            .find(&highscore.score.to_string())
            .is_some())
    }
}
