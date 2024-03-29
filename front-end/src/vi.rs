//! Converts key presses to commands
use crate::{Direction, GameStatus};
use futures::channel::mpsc;
use futures::stream::Stream;
use gloo_events::{EventListener, EventListenerOptions};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, KeyboardEvent};

pub struct Vi {
    pub receiver: mpsc::UnboundedReceiver<Command>,
    pub listener: EventListener,
}

#[derive(Debug)]
pub enum Command {
    Start,
    Stop,
    Help,
    Move(Direction),
}

impl Vi {
    pub fn new(target: &EventTarget, status: Arc<RwLock<GameStatus>>) -> Self {
        let (sender, receiver) = mpsc::unbounded();
        let listener = EventListener::new_with_options(
            target,
            "keydown",
            EventListenerOptions::enable_prevent_default(),
            move |event| {
                let event = event.dyn_ref::<KeyboardEvent>().unwrap();
                let key: &str = &event.key();
                let status = *status.read().unwrap();
                let dir = match key {
                    "h" | "ArrowLeft" if status == GameStatus::Playing => {
                        Some(Command::Move(Direction::Left))
                    }
                    "j" | "ArrowDown" if status == GameStatus::Playing => {
                        Some(Command::Move(Direction::Down))
                    }
                    "k" | "ArrowUp" if status == GameStatus::Playing => {
                        Some(Command::Move(Direction::Up))
                    }
                    "l" | "ArrowRight" if status == GameStatus::Playing => {
                        Some(Command::Move(Direction::Right))
                    }
                    " " if status != GameStatus::Playing => Some(Command::Start),
                    "q" if status == GameStatus::Playing => Some(Command::Stop),
                    "?" => Some(Command::Help),
                    _ => None,
                };

                if let Some(dir) = dir {
                    event.prevent_default();
                    sender.unbounded_send(dir).unwrap();
                };
            },
        );

        Self { receiver, listener }
    }
}

impl Stream for Vi {
    type Item = Command;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}
