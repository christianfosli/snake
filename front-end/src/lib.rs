//! Play snake using HTML canvas and web assembly.
//! Expects a html element with id=phone to exist, and renders the game into that element.
use futures::stream::StreamExt;
use gloo_dialogs::alert;
use gloo_timers::callback::Interval;
use gloo_utils::{document, window};
use js_sys::Error;
use std::fmt;
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

mod snake;

use crate::snake::{Direction, Snake};

mod render;

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

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = match self {
            Self::NotStarted => "Not started",
            Self::Playing => "Playing 🐍",
            Self::GameOver => "Game over",
        };
        write!(f, "{disp}")
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let fallback_url = || {
        log::debug!("Using current base url + /api as fallback url");
        window()
            .location()
            .origin()
            .map(|url| format!("{url}/api"))
            .unwrap_or_else(|_| {
                log::warn!("Unable to determine current base url. Trying with a relative one. This probably won't work.");
                String::from("/api")
            })
    };

    let highscore_url = match option_env!("HIGHSCORE_API_BASE_URL") {
        Some(url) if url.is_empty() => fallback_url(),
        Some(url) => String::from(url),
        None => fallback_url(),
    };

    log::debug!("Using highscore api base url {highscore_url}");
    {
        let base_url = highscore_url.clone();
        spawn_local(async move {
            let highscore_api = HighScoreApi::new(&base_url);
            highscores::fetch_and_set(&highscore_api)
                .await
                .unwrap_or_else(|err| log::error!("Unable to fetch highscores due to {err:?}"));
        });
    }

    let doc = document();
    let html_container: HtmlElement = doc
        .get_element_by_id("phone")
        .ok_or_else(|| Error::new("Could not find a phone element to mount snake into"))
        .map(JsCast::dyn_into)??;

    render::new_statusbar(&doc, &html_container)?;
    render::new_canvas(&doc, &html_container)?;

    let on_game_over = move |apples| {
        let base_url = highscore_url.clone();
        spawn_local(async move {
            let highscore_api = HighScoreApi::new(&base_url);
            game_over(&highscore_api, apples)
                .await
                .unwrap_or_else(|err| {
                    log::error!("End-of-Game actions failed due to {err:?}");
                });
        });
    };

    game_loop(on_game_over)?;

    Ok(())
}

fn game_loop<F: 'static>(on_game_over: F) -> Result<(), JsValue>
where
    F: Fn(u8),
{
    let snake = Snake::new();
    let status = Arc::new(RwLock::new(GameStatus::NotStarted));
    let dir = Arc::new(RwLock::new(snake.direction));
    let snake = Arc::new(RwLock::new(snake));

    let keylistener = {
        let snake = Arc::clone(&snake);
        let status = Arc::clone(&status);
        let dir = Arc::clone(&dir);

        async move {
            let doc = document();
            let mut vi = Vi::new(&doc, Arc::clone(&status));

            while let Some(cmd) = vi.next().await {
                match cmd {
                    Command::Start => {
                        let mut snake = snake.write().unwrap();
                        let mut game_status = status.write().unwrap();
                        if *game_status == GameStatus::GameOver {
                            *snake = Snake::new();
                            *dir.write().unwrap() = snake.direction;
                        }

                        *game_status = GameStatus::Playing;
                        render::update_statusbar(&doc, *game_status).unwrap_or_else(|e| {
                            log::error!("Failed to update game status due to {e:?}");
                        });

                        render::clear_canvas(&doc).unwrap_or_else(|e| {
                            log::error!("Failed to clear screen due to {e:?}");
                        });

                        render::snake(&doc, &snake)
                            .unwrap_or_else(|e| log::error!("Failed to draw snake due to {e:?}"));
                        render::apple(&doc, &snake.target.expect("target was undefined"))
                            .unwrap_or_else(|e| log::error!("Failed to draw apple due to {e:?}"));
                    }
                    Command::Stop => {
                        let mut snake = snake.write().unwrap();
                        *snake = snake.kill();
                    }
                    Command::Help => {
                        render::clear_canvas(&doc).unwrap();
                        render::text(
                            &doc,
                            "Navigate: hjkl
(like in vim),
arrow keys,
or phone keys

start: <space>
quit: <q>",
                            2,
                        )
                        .unwrap_or_else(|e| {
                            log::error!("Failed to write on canvas due to {e:?}");
                        });
                    }
                    Command::Move(d)
                        if snake.read().unwrap().apple_count() == 0
                            || d != snake.read().unwrap().direction.turn_180_degrees() =>
                    {
                        *dir.write().unwrap() = d;
                    }
                    fallback => {
                        log::debug!("Ignored command {fallback:?}");
                    }
                }
            }
        }
    };
    spawn_local(keylistener);

    let apple_counter: HtmlElement = document()
        .get_element_by_id("apple-counter")
        .map(JsCast::dyn_into)
        .ok_or_else(|| Error::new("Document had no apple counter"))??;

    let doc = document();

    Interval::new(300, move || {
        if *status.read().unwrap() != GameStatus::Playing {
            return;
        }

        let mut snake = snake.write().unwrap();
        snake.direction = *dir.read().unwrap();
        let (moved_snake, old_tail) = snake.move_along();
        *snake = moved_snake;

        if !snake.alive {
            *status.write().unwrap() = GameStatus::GameOver;
            on_game_over(snake.apple_count());
            return;
        }

        match old_tail {
            Some(tail) => render::clear_pos(&doc, &tail)
                .unwrap_or_else(|e| log::error!("Failed to clear tail due to {e:?}")),
            None if snake.target.is_some() => {
                render::apple(&doc, &snake.target.expect("target was undefined"))
                    .unwrap_or_else(|e| log::error!("Failed to draw apple due to {e:?}"));
            }
            None => render::text(&doc, "💯 u crazy!! 💯", 8)
                // The snake now covers the whole screen!
                .unwrap_or_else(|e| log::error!("Failed to write on canvas due to {e:?}")),
        }
        render::snake(&doc, &snake)
            .unwrap_or_else(|e| log::error!("Failed to draw snake due to {e:?}"));
        apple_counter.set_inner_text(&format!("🍎{}", snake.apple_count()));
    })
    .forget();

    Ok(())
}

async fn game_over(highscore_api: &HighScoreApi, apple_count: u8) -> Result<(), JsValue> {
    let doc = document();
    render::update_statusbar(&doc, GameStatus::GameOver)?;
    render::text(
        &doc,
        &format!(
            "score: {} {}",
            apple_count,
            if apple_count == 1 { "apple" } else { "apples" }
        ),
        4,
    )?;

    log::debug!("Checking if score {apple_count} is a highscore");
    match highscores::check_and_submit(highscore_api, apple_count).await {
        Ok(()) => {}
        Err(e) => {
            log::error!("{e:?}");
            alert(&format!("An error occured: {e}"));
        }
    }

    log::debug!("Refreshing highscore tables");
    highscores::fetch_and_set(highscore_api).await?;

    Ok(())
}
