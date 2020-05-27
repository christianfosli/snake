use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, HtmlElement, Request, RequestInit, RequestMode, Response};

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

#[wasm_bindgen(
    inline_js = "export function base_url() { return process.env.HIGHSCORE_API_BASE_URL; }"
)]
extern "C" {
    fn base_url() -> String;
}

pub async fn fetch_and_set_highscores() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let tbody = match window
        .document()
        .unwrap()
        .query_selector("#highscore-tbody")?
    {
        Some(v) => v.dyn_into::<HtmlElement>()?,
        None => panic!("no table body found!"),
    };

    let highscores = fetch_highscores().await;
    let html = match highscores {
        Ok(highscores) => highscores
            .iter()
            .map(|h| h.to_table_row())
            .collect::<String>(),
        Err(_) => String::from("<tr><td colspan=\"2\">Failed to fetch :-(</td></tr>"),
    };

    tbody.set_inner_html(&html);

    Ok(())
}

pub async fn fetch_highscores() -> Result<Vec<HighScore>, JsValue> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    let base_url = base_url();
    console::log_1(&format!("using highscore api url: {}", base_url).into());
    let endpoint = format!("{}/api/topten", base_url);

    let request = Request::new_with_str_and_init(&endpoint, &options)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let res: Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()?;

    let json = JsFuture::from(res.json()?).await?;
    let highscores: Vec<HighScore> = json.into_serde().unwrap();

    Ok(highscores)
}

pub async fn check_and_submit_highscore(score: usize) -> Result<(), JsValue> {
    let top_scores = fetch_highscores().await?;
    if top_scores.len() < 10 || top_scores.iter().any(|hs| hs.score < score) {
        console::log_1(&format!("Score {} is a highscore!", score).into());
        let window = web_sys::window().unwrap();
        let user_name =
            match window.prompt_with_message("Please enter your name for the highscore table")? {
                Some(v) => v,
                None => {
                    console::warn_1(&"highscore submission aborted as no username given".into());
                    return Ok(());
                }
            };

        let json = serde_json::to_string(&HighScore { user_name, score }).unwrap();

        let mut options = RequestInit::new();
        options.method("POST");
        options.mode(RequestMode::Cors);
        options.body(Some(&json.into()));

        let request =
            Request::new_with_str_and_init(&format!("{}/api/submit", base_url()), &options)?;

        request.headers().set("Accept", "application/json")?;
        request.headers().set("Content-Type", "text/plain")?;

        let res: Response = JsFuture::from(window.fetch_with_request(&request))
            .await?
            .dyn_into()?;

        match res.ok() {
            true => {
                console::log_1(&"highscore submitted".into());
                fetch_and_set_highscores().await?;
            }
            false => {
                console::error_1(&"failed to submit highscore".into());
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
