use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode, Response};

pub async fn fetch_and_set_highscores() -> Result<(), JsValue> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    let base_url = "http://localhost:7071"; // TODO: get from env variable
    let endpoint = format!("{}/api/HighScoreFetcher", base_url);

    let request = Request::new_with_str_and_init(&endpoint, &options)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let res: Response = JsFuture::from(window.fetch_with_request(&request))
        .await?
        .dyn_into()
        .unwrap();

    let json = JsFuture::from(res.json()?).await?;
    let highscores: Vec<HighScore> = json.into_serde().unwrap();
    let html: String = highscores.iter().map(|h| h.to_html_row()).collect();

    let tbody = match window
        .document()
        .unwrap()
        .query_selector("#highscore-tbody")?
    {
        Some(v) => v.dyn_into::<HtmlElement>()?,
        None => panic!("no table body found!"),
    };

    tbody.set_inner_html(&html);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)] // weird capitalization to match js conventions
struct HighScore {
    userName: String,
    score: usize,
}

impl HighScore {
    fn to_html_row(&self) -> String {
        format!("<tr><td>{}</td><td>{}</td></tr>", self.userName, self.score)
    }
}
