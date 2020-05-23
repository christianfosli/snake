use futures::stream::StreamExt;
use gloo_timers::callback::Interval;
use serde::{Deserialize, Serialize};
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    console, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement, Request, RequestInit,
    RequestMode, Response,
};

mod snake;
use crate::snake::*;

mod vi;
use crate::vi::*;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    spawn_local(async {
        fetch_and_set_highscores()
            .await
            .unwrap_or_else(|err| console::log_1(&err.into()))
    });
    add_canvas()?;

    let mut snake = Snake::new();
    draw_snake(&snake)?;
    draw_apple(&snake.target)?;

    let direction_ptr = Arc::new(Mutex::new(snake.direction));
    let direction_ptr_2 = Arc::clone(&direction_ptr);
    let interval_ptr = Arc::new(Mutex::new(0));
    let interval_ptr_2 = Arc::clone(&interval_ptr);

    let keylistener = async move {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut vi = Vi::new(&document);

        while let Some(dir) = vi.next().await {
            *direction_ptr_2.lock().unwrap() = dir;
        }
    };

    spawn_local(keylistener);

    *interval_ptr.lock().unwrap() = Interval::new(300, move || {
        snake.direction = *direction_ptr.lock().unwrap();
        let (moved_snake, old_tail) = snake.move_along();
        snake = moved_snake;

        if !snake.alive {
            web_sys::window()
                .unwrap()
                .clear_interval_with_handle(*interval_ptr_2.lock().unwrap());
            return;
        }

        draw_snake(&snake).unwrap();
        match old_tail {
            Some(tail) => clear(&tail).unwrap(),
            None => draw_apple(&snake.target).unwrap(),
        }
    })
    .forget();

    Ok(())
}

async fn fetch_and_set_highscores() -> Result<(), JsValue> {
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
struct HighScore {
    userName: String, // weird capitalization to match C# conventions
    score: usize,
}

impl HighScore {
    fn to_html_row(&self) -> String {
        format!("<tr><td>{}</td><td>{}</td></tr>", self.userName, self.score)
    }
}

fn add_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let main_section = match document.query_selector("#phone")? {
        Some(v) => v.dyn_into::<HtmlElement>()?,
        None => document.body().unwrap(),
    };

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_id("canvas");
    canvas.set_width(snake::WIDTH);
    canvas.set_height(snake::HEIGHT);

    main_section.insert_adjacent_element("afterbegin", &canvas)?;

    Ok(())
}

fn get_canvas_context() -> Result<CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .expect("no canvas element could be found")
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok(context)
}

fn draw_snake(snake: &Snake) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.set_fill_style(&JsValue::from_str("#bada55"));
    for pos in snake.body.iter() {
        context.fill_rect(pos.x, pos.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    }
    Ok(())
}

fn draw_apple(apple: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    let radius = (snake::LINE_THICKNESS / 2.0).floor();
    let x = (apple.x + snake::LINE_THICKNESS / 2.0).round();
    let y = (apple.y + snake::LINE_THICKNESS / 2.0).round();

    context.set_fill_style(&JsValue::from_str("red"));
    context.begin_path();
    context.ellipse(x, y, radius, radius, PI / 4.0, 0.0, 2.0 * PI)?;
    context.fill();
    context.close_path();
    Ok(())
}

fn clear(rect: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.clear_rect(rect.x, rect.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    Ok(())
}
