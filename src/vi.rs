use crate::Direction;
use futures::channel::mpsc;
use futures::stream::Stream;
use gloo_events::EventListener;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, KeyboardEvent};

pub struct Vi {
    pub receiver: mpsc::UnboundedReceiver<Direction>,
    pub listener: EventListener,
}

impl Vi {
    pub fn new(target: &EventTarget) -> Self {
        let (sender, receiver) = mpsc::unbounded();
        let listener = EventListener::new(&target, "keydown", move |event| {
            let event: &KeyboardEvent = event.dyn_ref::<KeyboardEvent>().unwrap();
            let key: &str = &event.key();
            let dir: Option<Direction> = match key {
                "h" => Some(Direction::Left),
                "j" => Some(Direction::Down),
                "k" => Some(Direction::Up),
                "l" => Some(Direction::Right),
                _ => None,
            };
            match dir {
                Some(dir) => sender.unbounded_send(dir).unwrap(),
                None => (),
            };
        });

        Self { receiver, listener }
    }
}

impl Stream for Vi {
    type Item = Direction;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}
