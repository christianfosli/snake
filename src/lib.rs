extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

pub struct Snake {
    body: Vec<Position>,
    direction: Direction,
    thickness: f64,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: vec![Position { x: 0.0, y: 0.0 }],
            direction: Direction::Right,
            thickness: 25.0,
        }
    }
    fn next_position(&self) -> Position {
        let head = self.body.first().unwrap();
        match self.direction {
            Direction::Right => Position {
                x: head.x + self.thickness,
                ..*head
            },
            Direction::Down => Position {
                y: head.y + self.thickness,
                ..*head
            },
            Direction::Left => Position {
                x: head.x - self.thickness,
                ..*head
            },
            Direction::Up => Position {
                y: head.y - self.thickness,
                ..*head
            },
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
}

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    add_canvas()?;
    let snake = Snake::new();
    draw_snake(&snake)?;

    Ok(())
}

fn add_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlElement>()?;
    canvas.set_id("canvas");

    body.append_child(&canvas)?;

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
    for pos in snake.body.iter() {
        context.fill_rect(pos.x, pos.y, snake.thickness, snake.thickness);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initially_moves_right() {
        let snake = Snake::new();
        let initial_dir = &snake.direction;
        assert!(matches!(Direction::Right, initial_dir));
    }

    #[test]
    fn next_position_is_thickness_away_from_head() {
        let snake = Snake::new();
        let head = snake.body.first().expect("snake should have a body");
        let next = snake.next_position();
        assert_eq!(next.x, head.x + snake.thickness);
    }
}
