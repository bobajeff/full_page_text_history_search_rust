pub mod browser_operations;
pub mod index_management;
pub use browser_operations::connect_to_browser::connect_to_browser;
pub use index_management::init;

use chrono::prelude::*;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::task::Poll;
use futures::Stream;
use std::pin::Pin;

pub use index_management::entry_builder::EntryData;

pub struct Entry {
    pub timestamp: i64,
    pub address: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

pub struct ProtoEntry {
    pub timestamp: i64,
    pub address: String,
    pub title: String,
    pub text: String,
}

impl ProtoEntry {
    pub fn new() -> Self {
        Self {
            timestamp: Local::now().timestamp(),
            address: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
pub struct EntryStream {
    receiver: Receiver<Entry>,
}

impl EntryStream {
    pub fn new(receiver: Receiver<Entry>) -> Self {
        Self { receiver }
    }
}

impl Stream for EntryStream {
    type Item = Entry;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let pin = self.get_mut();
        match Stream::poll_next(Pin::new(&mut pin.receiver), cx) {
            Poll::Ready(Some(event)) => Poll::Ready(Some(event)),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
