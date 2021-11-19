use futures::stream::StreamExt;
use gloo_timers::callback::Interval;
use js_sys::Error;
use std::{
    f64::consts::PI,
    sync::{Arc, RwLock},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement};

mod snake;
use crate::snake::{Direction, Position, Snake};

mod vi;
use crate::vi::{Command, Vi};

mod highscores;

mod services;
use crate::services::highscore_api::HighScoreApi;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameStatus {
    NotStarted,
    Playing,
    GameOver,
}

// Called by our JS entry point
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let highscore_base_url = option_env!("HIGHSCORE_API_BASE_URL").unwrap_or("");
    log::debug!("Using highscore api base url {:?}", &highscore_base_url);

    spawn_local(async move {
        let highscore_api = HighScoreApi::new(highscore_base_url.to_owned());
        highscores::fetch_and_set(&highscore_api)
            .await
            .unwrap_or_else(|err| log::error!("Unable to fetch highscores due to {:?}", &err));
    });

    let document = web_sys::window()
        .ok_or_else(|| Error::new("Window was none"))?
        .document()
        .ok_or_else(|| Error::new("Window had no document"))?;

    let html_container: HtmlElement = document
        .get_element_by_id("phone")
        .ok_or_else(|| Error::new("Could not find a phone element to mount snake into"))
        .map(JsCast::dyn_into)??;

    add_statusbar(&document, &html_container)?;
    add_canvas(&document, &html_container)?;

    let (status_ptr, status_ptr_2, dir_ptr, dir_ptr_2, snake_ptr, snake_ptr_2) = {
        let snake = Snake::new();
        let status = Arc::new(RwLock::new(GameStatus::NotStarted));
        let status_2 = Arc::clone(&status);
        let dir = Arc::new(RwLock::new(snake.direction));
        let dir_2 = Arc::clone(&dir);
        let snake = Arc::new(RwLock::new(snake));
        let snake_2 = Arc::clone(&snake);
        (status, status_2, dir, dir_2, snake, snake_2)
    };

    let keylistener = async move {
        let mut vi = Vi::new(&document, Arc::clone(&status_ptr_2));

        while let Some(cmd) = vi.next().await {
            match cmd {
                Command::Start => {
                    let mut snake = snake_ptr_2.write().unwrap();
                    let mut game_status = status_ptr_2.write().unwrap();
                    if *game_status == GameStatus::GameOver {
                        *snake = Snake::new();
                        *dir_ptr_2.write().unwrap() = snake.direction;
                    }

                    *game_status = GameStatus::Playing;
                    update_status_in_statusbar(&document, *game_status).unwrap_or_else(|e| {
                        log::error!("Failed to update game status due to {:?}", e);
                    });

                    clear_screen(&document)
                        .unwrap_or_else(|e| log::error!("Failed to clear screen due to {:?}", e));

                    draw_snake(&document, &snake)
                        .unwrap_or_else(|e| log::error!("Failed to draw snake due to {:?}", e));
                    draw_apple(&document, &snake.target.expect("target was undefined"))
                        .unwrap_or_else(|e| log::error!("Failed to draw apple due to {:?}", e));
                }
                Command::Stop => {
                    let mut snake = snake_ptr_2.write().unwrap();
                    *snake = snake.kill();
                }
                Command::Help => {
                    clear_screen(&document).unwrap();
                    write_on_canvas(&document, "Navigate:", 2).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                    write_on_canvas(&document, "hjkl, ⬅⬇⬆️️➡️️, or", 3).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                    write_on_canvas(&document, "the numpad below", 4).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                    write_on_canvas(&document, "start with", 6).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                    write_on_canvas(&document, "<space>", 7).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                    write_on_canvas(&document, "quit with <q>", 9).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e);
                    });
                }
                Command::Move(dir)
                    if snake_ptr_2.read().unwrap().apple_count() == 0
                        || dir != snake_ptr_2.read().unwrap().direction.turn_180_degrees() =>
                {
                    *dir_ptr_2.write().unwrap() = dir;
                }
                fallback => {
                    log::debug!("Ignored command {:?}", fallback);
                }
            }
        }
    };

    spawn_local(keylistener);

    let document = web_sys::window()
        .ok_or_else(|| Error::new("Window was none"))?
        .document()
        .ok_or_else(|| Error::new("Window had no document"))?;

    let apple_counter: HtmlElement = document
        .get_element_by_id("apple-counter")
        .map(JsCast::dyn_into)
        .ok_or_else(|| Error::new("Document had no apple counter"))??;

    Interval::new(300, move || {
        if *status_ptr.read().unwrap() != GameStatus::Playing {
            return;
        }

        let mut snake = snake_ptr.write().unwrap();
        snake.direction = *dir_ptr.read().unwrap();
        let (moved_snake, old_tail) = snake.move_along();
        *snake = moved_snake;

        if !snake.alive {
            let dead_snake = Snake {
                // just creating a copy so we can move it into async fn
                body: snake.body.clone(),
                ..*snake
            };
            *status_ptr.write().unwrap() = GameStatus::GameOver;

            let document = document.clone();

            spawn_local(async move {
                let highscore_api = HighScoreApi::new(highscore_base_url.into());

                game_over(&document, &highscore_api, &dead_snake)
                    .await
                    .unwrap_or_else(|err| {
                        log::error!("End-of-Game actions failed due to {:?}", err);
                    });
            });
            return;
        }

        match old_tail {
            Some(tail) => clear(&document, &tail)
                .unwrap_or_else(|e| log::error!("Failed to clear tail due to {:?}", e)),
            None if snake.target.is_some() => {
                draw_apple(&document, &snake.target.expect("target was undefined"))
                    .unwrap_or_else(|e| log::error!("Failed to draw apple due to {:?}", e));
            }
            None => write_on_canvas(&document, "💯 u crazy!! 💯", 8)
                // The snake now covers the whole screen!
                .unwrap_or_else(|e| log::error!("Failed to write on canvas due to {:?}", e)),
        }
        draw_snake(&document, &snake)
            .unwrap_or_else(|e| log::error!("Failed to draw snake due to {:?}", e));
        apple_counter.set_inner_text(&format!("🍎{}", snake.apple_count()));
    })
    .forget();

    Ok(())
}

async fn game_over(
    document: &Document,
    highscore_api: &HighScoreApi,
    snake: &Snake,
) -> Result<(), JsValue> {
    let apple_count = snake.apple_count();
    log::debug!("Game over with {} apples eaten", apple_count);

    update_status_in_statusbar(document, GameStatus::GameOver)?;

    write_on_canvas(
        document,
        &format!(
            "score: {} {}",
            apple_count,
            match apple_count {
                1 => "apple",
                _ => "apples",
            }
        ),
        4,
    )?;

    log::debug!("Checking if score is a highscore");
    highscores::check_and_submit(highscore_api, apple_count).await?;

    log::debug!("Refreshing highscore tables");
    highscores::fetch_and_set(highscore_api).await?;

    Ok(())
}

fn add_canvas(document: &Document, parent: &HtmlElement) -> Result<(), JsValue> {
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_id("canvas");
    canvas.set_width(snake::WIDTH);
    canvas.set_height(snake::HEIGHT);

    let insert_after: HtmlElement = parent
        .query_selector(".statusbar")?
        .map(|el| el.dyn_into().unwrap())
        .ok_or_else(|| Error::new("No statusbar found to insert canvas under"))?;

    insert_after.insert_adjacent_element("afterend", &canvas)?;

    Ok(())
}

fn get_canvas_context(document: &Document) -> Result<CanvasRenderingContext2d, JsValue> {
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

fn draw_snake(document: &Document, snake: &Snake) -> Result<(), JsValue> {
    let context = get_canvas_context(document)?;
    context.set_fill_style(&JsValue::from_str("#abba00"));
    context.fill_rect(
        snake.head().x,
        snake.head().y,
        snake::LINE_THICKNESS,
        snake::LINE_THICKNESS,
    );
    snake.body.iter().rev().nth(1).map(|next| {
        context.set_fill_style(&JsValue::from_str("#bada55"));
        context.fill_rect(next.x, next.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
        next
    });
    Ok(())
}

fn draw_apple(document: &Document, apple: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context(document)?;
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

fn clear(document: &Document, rect: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context(document)?;
    context.clear_rect(rect.x, rect.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    Ok(())
}

fn clear_screen(document: &Document) -> Result<(), JsValue> {
    let context = get_canvas_context(document)?;
    context.clear_rect(0.0, 0.0, f64::from(snake::WIDTH), f64::from(snake::HEIGHT));
    Ok(())
}

fn write_on_canvas(document: &Document, text: &str, row: u8) -> Result<(), JsValue> {
    let context = get_canvas_context(document)?;
    context.set_font("30px monospace");
    context.set_fill_style(&"blue".into());
    context.fill_text(text, 10.0, f64::from(row) * snake::LINE_THICKNESS)?;
    Ok(())
}

fn add_statusbar(document: &Document, parent: &HtmlElement) -> Result<(), JsValue> {
    let statusbar = document.create_element("div")?.dyn_into::<HtmlElement>()?;

    statusbar.set_class_name("statusbar");

    statusbar.set_inner_html(
        "<span id=\"apple-counter\">🍎0</span>\n\
         <span id=\"game-status\"></span>\n",
    );

    parent.insert_adjacent_element("afterbegin", &statusbar)?;

    update_status_in_statusbar(document, GameStatus::NotStarted)?;

    Ok(())
}

fn update_status_in_statusbar(document: &Document, status: GameStatus) -> Result<(), JsValue> {
    let game_status_element: HtmlElement = document
        .query_selector("#game-status")?
        .map(JsCast::dyn_into)
        .ok_or_else(|| Error::new("Document had no game status element"))??;

    let status_text = match status {
        GameStatus::NotStarted => "Press <space> to start",
        GameStatus::GameOver => "Restart with <space>",
        GameStatus::Playing => "Playing 🐍",
    };

    game_status_element.set_inner_text(status_text);

    Ok(())
}
