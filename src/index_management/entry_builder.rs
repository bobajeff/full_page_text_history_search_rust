use async_std::task::TaskId;
use futures::StreamExt;
use futures::channel::mpsc::{Receiver, Sender};
use futures::Stream;
use std::pin::Pin;
use std::collections::HashMap;
use futures::task::Poll;

use crate::{Entry, ProtoEntry};

pub enum EntryData {
    Timestamp(i64),
    Address(String),
    Title(String),
    Text(String),
}

pub struct EntryDataStream {
    receiver: Receiver<(TaskId, EntryData)>,
}

impl EntryDataStream {
    pub fn new(receiver: Receiver<(TaskId, EntryData)>) -> Self {
        Self { receiver }
    }
}

impl Stream for EntryDataStream {
    type Item = (TaskId, EntryData);

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

pub fn start_entry_manager(rx: Receiver<(TaskId, EntryData)>, finished_entry_sender: Sender<Entry>){
    let mut entries: HashMap<TaskId, ProtoEntry> = HashMap::new();
    let mut entry_data_stream = EntryDataStream::new(rx);

    async_std::task::spawn(async move {
        while let Some(entry_data) = entry_data_stream.next().await {
            let entry = entries.entry(entry_data.0).or_insert_with(ProtoEntry::new);
            match entry_data.1 {
                EntryData::Timestamp(timestamp) => {entry.timestamp = timestamp;},
                EntryData::Address(address) => {entry.address = address;},
                EntryData::Title(title) => {entry.title = title;},
                EntryData::Text(text) => {entry.text += &text;}
            }
        }
    });

}