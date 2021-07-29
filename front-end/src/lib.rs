use futures::stream::StreamExt;
use gloo_timers::callback::Interval;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

mod snake;
use crate::snake::*;

mod vi;
use crate::vi::*;

mod highscores;
use crate::highscores::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameStatus {
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
        fetch_and_set_highscores(&highscore_base_url)
            .await
            .unwrap_or_else(|err| log::error!("Unable to fetch highscores due to {:?}", &err))
    });
    add_canvas()?;
    write_on_canvas("Press <space>", 3)?;
    write_on_canvas("to begin", 4)?;

    let snake = Snake::new();

    let game_status_ptr = Arc::new(Mutex::new(GameStatus::NotStarted));
    let game_status_ptr_2 = Arc::clone(&game_status_ptr);
    let direction_ptr = Arc::new(Mutex::new(snake.direction));
    let direction_ptr_2 = Arc::clone(&direction_ptr);
    let snake_ptr = Arc::new(Mutex::new(snake));
    let snake_ptr_2 = Arc::clone(&snake_ptr);

    let keylistener = async move {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut vi = Vi::new(&document);

        while let Some(cmd) = vi.next().await {
            let mut snake = snake_ptr_2.lock().unwrap();
            match cmd {
                ViCommand::Start => {
                    let mut game_status = game_status_ptr.lock().unwrap();
                    if *game_status == GameStatus::GameOver {
                        *snake = Snake::new();
                        *direction_ptr_2.lock().unwrap() = snake.direction;
                    }
                    *game_status = GameStatus::Playing;
                    clear_screen()
                        .unwrap_or_else(|e| log::error!("Failed to clear screen due to {:?}", e));
                    draw_snake(&snake)
                        .unwrap_or_else(|e| log::error!("Failed to draw snake due to {:?}", e));
                    draw_apple(&snake.target.expect("target was undefined"))
                        .unwrap_or_else(|e| log::error!("Failed to draw apple due to {:?}", e));
                }
                ViCommand::Stop if *game_status_ptr.lock().unwrap() == GameStatus::Playing => {
                    *snake = snake.kill();
                }
                ViCommand::Help => {
                    clear_screen().unwrap();
                    write_on_canvas("Navigate:", 2).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                    write_on_canvas("hjkl, ⬅⬇⬆️️➡️️, or", 3).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                    write_on_canvas("the numpad below", 4).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                    write_on_canvas("start with", 6).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                    write_on_canvas("<space>", 7).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                    write_on_canvas("quit with <q>", 9).unwrap_or_else(|e| {
                        log::error!("Failed to write on canvas due to {:?}", e)
                    });
                }
                ViCommand::Move(dir)
                    if snake.apple_count() == 0 || dir != snake.direction.turn_180_degrees() =>
                {
                    *direction_ptr_2.lock().unwrap() = dir;
                }
                _ => {}
            }
        }
    };

    spawn_local(keylistener);

    Interval::new(300, move || {
        let mut game_status = game_status_ptr_2.lock().unwrap();
        if *game_status != GameStatus::Playing {
            return;
        }

        let mut snake = snake_ptr.lock().unwrap();
        snake.direction = *direction_ptr.lock().unwrap();
        let (moved_snake, old_tail) = snake.move_along();
        *snake = moved_snake;

        if !snake.alive {
            let dead_snake = Snake {
                // just creating a copy so we can move it into async fn
                body: snake.body.clone(),
                ..*snake
            };
            *game_status = GameStatus::GameOver;
            spawn_local(async move {
                game_over(&highscore_base_url, &dead_snake)
                    .await
                    .unwrap_or_else(|err| {
                        log::error!("End-of-Game actions failed due to {:?}", err)
                    });
            });
            return;
        }

        match old_tail {
            Some(tail) => {
                clear(&tail).unwrap_or_else(|e| log::error!("Failed to clear tail due to {:?}", e))
            }
            None if snake.target.is_some() => {
                draw_apple(&snake.target.expect("target was undefined"))
                    .unwrap_or_else(|e| log::error!("Failed to draw apple due to {:?}", e))
            }
            None => write_on_canvas("💯 u crazy!! 💯", 8)
                // The snake now covers the whole screen!
                .unwrap_or_else(|e| log::error!("Failed to write on canvas due to {:?}", e)),
        }
        draw_snake(&snake).unwrap_or_else(|e| log::error!("Failed to draw snake due to {:?}", e));
    })
    .forget();

    Ok(())
}

async fn game_over(highscore_url: &str, snake: &Snake) -> Result<(), JsValue> {
    write_on_canvas(
        &format!(
            "score: {} {}",
            snake.apple_count(),
            match snake.apple_count() {
                1 => "apple",
                _ => "apples",
            }
        ),
        4,
    )?;

    check_and_submit_highscore(highscore_url, snake.apple_count()).await?;

    Ok(())
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

fn clear_screen() -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.clear_rect(0.0, 0.0, snake::WIDTH as f64, snake::HEIGHT as f64);
    Ok(())
}

fn write_on_canvas(text: &str, row: u8) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.set_font("30px monospace");
    context.set_fill_style(&"blue".into());
    context.fill_text(text, 10.0, row as f64 * snake::LINE_THICKNESS)?;
    Ok(())
}
