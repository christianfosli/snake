//! Utilities for drawing/rendering snake on a fake phone screen
use std::f64::consts::PI;

use js_sys::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement};

use crate::{
    snake::{self, Position, Snake},
    GameStatus,
};

pub fn new_canvas(doc: &Document, parent: &HtmlElement) -> Result<(), JsValue> {
    let canvas = doc
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

    text(doc, "Press <space>\nto start\n\nPress <?> for\nhelp", 2)?;

    Ok(())
}

pub fn snake(doc: &Document, snake: &Snake) -> Result<(), JsValue> {
    let context = get_canvas_context(doc)?;
    context.set_fill_style_str("#abba00");
    context.fill_rect(
        snake.head().x,
        snake.head().y,
        snake::LINE_THICKNESS,
        snake::LINE_THICKNESS,
    );
    snake.body.iter().rev().nth(1).inspect(|next| {
        context.set_fill_style_str("#bada55");
        context.fill_rect(next.x, next.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    });
    Ok(())
}

pub fn apple(doc: &Document, apple: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context(doc)?;
    let radius = (snake::LINE_THICKNESS / 2.0).floor();
    let x = (apple.x + snake::LINE_THICKNESS / 2.0).round();
    let y = (apple.y + snake::LINE_THICKNESS / 2.0).round();

    context.set_fill_style_str("red");
    context.begin_path();
    context.ellipse(x, y, radius, radius, PI / 4.0, 0.0, 2.0 * PI)?;
    context.fill();
    context.close_path();
    Ok(())
}

pub fn clear_pos(doc: &Document, rect: &Position) -> Result<(), JsValue> {
    let context = get_canvas_context(doc)?;
    context.clear_rect(rect.x, rect.y, snake::LINE_THICKNESS, snake::LINE_THICKNESS);
    Ok(())
}

pub fn clear_canvas(doc: &Document) -> Result<(), JsValue> {
    let context = get_canvas_context(doc)?;
    context.clear_rect(0.0, 0.0, f64::from(snake::WIDTH), f64::from(snake::HEIGHT));
    Ok(())
}

pub fn text(doc: &Document, txt: &str, row: u8) -> Result<(), JsValue> {
    if txt.contains('\n') {
        txt.lines()
            .enumerate()
            .try_for_each(|(i, l)| text(doc, l, row + i as u8))?;
        return Ok(());
    }

    let context = get_canvas_context(doc)?;
    context.set_font("30px monospace");
    context.set_fill_style_str("blue");
    context.fill_text(txt, 10.0, f64::from(row) * snake::LINE_THICKNESS)?;
    Ok(())
}

pub fn new_statusbar(doc: &Document, parent: &HtmlElement) -> Result<(), JsValue> {
    let statusbar = doc.create_element("div")?.dyn_into::<HtmlElement>()?;

    statusbar.set_class_name("statusbar");

    statusbar.set_inner_html(
        "<span id=\"apple-counter\">🍎0</span>\n\
         <span id=\"game-status\"></span>\n",
    );

    parent.insert_adjacent_element("afterbegin", &statusbar)?;

    update_statusbar(doc, GameStatus::NotStarted)?;

    Ok(())
}

pub fn update_statusbar(doc: &Document, status: GameStatus) -> Result<(), JsValue> {
    let game_status_element: HtmlElement = doc
        .query_selector("#game-status")?
        .map(JsCast::dyn_into)
        .ok_or_else(|| Error::new("Document had no game status element"))??;

    game_status_element.set_inner_text(&format!("{status}"));

    Ok(())
}

fn get_canvas_context(doc: &Document) -> Result<CanvasRenderingContext2d, JsValue> {
    let canvas = doc
        .get_element_by_id("canvas")
        .expect("no canvas element could be found")
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok(context)
}
