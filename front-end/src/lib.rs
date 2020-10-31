use futures::stream::StreamExt;
use gloo_timers::callback::Interval;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

mod snake;
use crate::snake::*;

mod vi;
use crate::vi::*;

mod highscores;
use crate::highscores::*;

// Called by our JS entry point
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    spawn_local(async {
        fetch_and_set_highscores()
            .await
            .unwrap_or_else(|err| console::error_1(&err.into()))
    });
    add_canvas()?;

    let snake = Snake::new();
    draw_snake(&snake)?;
    draw_apple(&snake.target.unwrap())?;

    let direction_ptr = Arc::new(Mutex::new(snake.direction));
    let direction_ptr_2 = Arc::clone(&direction_ptr);
    let snake_ptr = Arc::new(Mutex::new(snake));
    let snake_ptr_2 = Arc::clone(&snake_ptr);
    let interval_ptr = Arc::new(Mutex::new(0));
    let interval_ptr_2 = Arc::clone(&interval_ptr);

    let keylistener = async move {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut vi = Vi::new(&document);

        while let Some(new_dir) = vi.next().await {
            let snake = snake_ptr_2.lock().unwrap();
            if snake.apple_count() == 0 || new_dir != snake.direction.turn_180_degrees() {
                *direction_ptr_2.lock().unwrap() = new_dir;
            }
        }
    };

    spawn_local(keylistener);

    *interval_ptr.lock().unwrap() = Interval::new(300, move || {
        let mut snake = snake_ptr.lock().unwrap();
        snake.direction = *direction_ptr.lock().unwrap();
        let (moved_snake, old_tail) = snake.move_along();
        *snake = moved_snake;

        if !snake.alive {
            let interval_handle = *interval_ptr_2.lock().unwrap();
            let dead_snake = Snake {
                // just creating a copy so we can move it into async fn
                body: snake.body.clone(),
                ..*snake
            };
            spawn_local(async move {
                game_over(&dead_snake, interval_handle)
                    .await
                    .unwrap_or_else(|err| console::error_1(&err.into()));
            });
            return;
        }

        match old_tail {
            Some(tail) => clear(&tail).unwrap(),
            None if snake.target.is_some() => draw_apple(&snake.target.unwrap()).unwrap(),
            None => write_on_canvas("💯 u crazy!! 💯", 8).unwrap(),
        }
        draw_snake(&snake).unwrap();
    })
    .forget();

    Ok(())
}

async fn game_over(snake: &Snake, interval_handle: i32) -> Result<(), JsValue> {
    web_sys::window()
        .unwrap()
        .clear_interval_with_handle(interval_handle);

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

    check_and_submit_highscore(snake.apple_count()).await?;

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
    snake.body.iter().rev().skip(1).next().and_then(|next| {
        context.set_fill_style(&JsValue::from_str("#bada55"));
        context.fill_rect(next.x, next.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
        Some(next)
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

fn write_on_canvas(text: &str, row: u8) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.set_font("30px monospace");
    context.set_fill_style(&"blue".into());
    context.fill_text(text, 10.0, row as f64 * snake::LINE_THICKNESS)?;
    Ok(())
}
