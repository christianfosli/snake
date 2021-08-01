use chrono::prelude::*;
use js_sys::{Date, Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

use crate::services::highscore_api::HighScoreApi;

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

pub async fn fetch_and_set_highscores(client: &HighScoreApi) -> Result<(), JsValue> {
    let dom = web_sys::window()
        .ok_or_else(|| Error::new("Window was none"))?
        .document()
        .ok_or_else(|| Error::new("Window contains no document"))?;

    let topten_alltime_fut = client.top_ten(None);

    let start_of_year = Utc
        .ymd(Date::new_0().get_utc_full_year() as i32, 1, 1)
        .and_hms(0, 0, 0);

    let topten_yearly_fut = client.top_ten(Some(start_of_year));

    let topten_alltime_html = topten_alltime_fut
        .await
        .map(|hs| hs.iter().map(|hs| hs.to_table_row()).collect::<String>())
        .unwrap_or_else(|err| {
            log::error!("Error fetching top ten alltime: {:?}", err);
            String::from("<tr><td colspan=\"2\">Failed to fetch top ten alltime 😩</td></tr>")
        });

    dom.query_selector("#topten-alltime tbody")?
        .ok_or_else(|| Error::new("Cant find topten alltime table"))
        .map(|table| table.dyn_into::<HtmlElement>())?
        .map(|table| table.set_inner_html(&topten_alltime_html))?;

    let topten_yearly_html = topten_yearly_fut
        .await
        .map(|hs| hs.iter().map(|hs| hs.to_table_row()).collect::<String>())
        .unwrap_or_else(|err| {
            log::error!("Error fetching top ten yearly: {:?}", err);
            String::from("<tr><td colspan=\"2\">Failed to fetch top ten this year 😩</td></tr>")
        });

    dom.query_selector("#topten-yearly tbody")?
        .ok_or_else(|| Error::new("Cant find topten yearly table"))
        .map(|table| table.dyn_into::<HtmlElement>())?
        .map(|table| table.set_inner_html(&topten_yearly_html))?;

    Ok(())
}

pub async fn check_and_submit_highscore(
    client: &HighScoreApi,
    score: usize,
) -> Result<(), JsValue> {
    let top_scores = client.top_ten(None).await?;
    if top_scores.len() < 10 || top_scores.iter().any(|hs| hs.score < score) {
        log::debug!("Score {} is a highscore!", score);

        let window = web_sys::window().ok_or_else(|| Error::new("Window was none"))?;
        let highscore = window
            .prompt_with_message("Please enter your name for the highscore table")?
            .map(|user_name| HighScore { user_name, score });

        match &highscore {
            Some(hs) => client.submit(hs).await?,
            None => log::warn!("highscore submission aborted because no username given"),
        };
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
