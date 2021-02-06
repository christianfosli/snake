use crate::Direction;
use futures::channel::mpsc;
use futures::stream::Stream;
use gloo_events::EventListener;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, KeyboardEvent};

pub struct Vi {
    pub receiver: mpsc::UnboundedReceiver<ViCommand>,
    pub listener: EventListener,
}

pub enum ViCommand {
    Start,
    Stop,
    Help,
    Move(Direction),
}

impl Vi {
    pub fn new(target: &EventTarget) -> Self {
        let (sender, receiver) = mpsc::unbounded();
        let listener = EventListener::new(&target, "keydown", move |event| {
            let event: &KeyboardEvent = event.dyn_ref::<KeyboardEvent>().unwrap();
            let key: &str = &event.key();
            let dir: Option<ViCommand> = match key {
                "h" | "ArrowLeft" => Some(ViCommand::Move(Direction::Left)),
                "j" | "ArrowDown" => Some(ViCommand::Move(Direction::Down)),
                "k" | "ArrowUp" => Some(ViCommand::Move(Direction::Up)),
                "l" | "ArrowRight" => Some(ViCommand::Move(Direction::Right)),
                " " => Some(ViCommand::Start),
                "q" => Some(ViCommand::Stop),
                "?" => Some(ViCommand::Help),
                _ => None,
            };

            if let Some(dir) = dir {
                sender.unbounded_send(dir).unwrap()
            };
        });

        Self { receiver, listener }
    }
}

impl Stream for Vi {
    type Item = ViCommand;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}
