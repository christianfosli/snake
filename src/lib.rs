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

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    add_canvas()?;

    let mut snake = Snake::new();
    draw_snake(&snake)?;
    draw_apple(&snake.target)?;

    let direction_ptr = Arc::new(Mutex::new(snake.direction));
    let direction_ptr_2 = Arc::clone(&direction_ptr);
    let interval_ptr = Arc::new(Mutex::new(0));
    let interval_ptr_2 = Arc::clone(&interval_ptr);

    let fut = async move {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut vi = Vi::new(&document);

        while let Some(dir) = vi.next().await {
            *direction_ptr_2.lock().unwrap() = dir;
        }
    };

    spawn_local(fut);

    *interval_ptr.lock().unwrap() = Interval::new(500, move || {
        snake.direction = *direction_ptr.lock().unwrap();
        let (moved_snake, old_tail) = snake.move_along();

        if !snake.alive {
            web_sys::window()
                .unwrap()
                .clear_interval_with_handle(*interval_ptr_2.lock().unwrap());
            return;
        }

        draw_snake(&moved_snake).unwrap();
        match old_tail {
            Some(tail) => clear(&tail).unwrap(),
            None => draw_apple(&moved_snake.target).unwrap(),
        }
        snake = moved_snake;
    })
    .forget();

    Ok(())
}

fn add_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let main_section = match document.query_selector("main")? {
        Some(v) => v.dyn_into::<HtmlElement>()?,
        None => document.body().unwrap(),
    };

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    canvas.set_id("canvas");
    canvas.set_width(snake::WIDTH);
    canvas.set_height(snake::HEIGHT);

    main_section.append_child(&canvas)?;

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
    let radius = (snake::LINE_THICKNESS / 2.0).round();
    context.set_fill_style(&JsValue::from_str("red"));
    context.begin_path();
    context.ellipse(apple.x, apple.y, radius, radius, PI / 4.0, 0.0, 2.0 * PI)?;
    context.fill();
    context.close_path();
    Ok(())
}

fn clear(rect: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.clear_rect(rect.x, rect.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    Ok(())
}
