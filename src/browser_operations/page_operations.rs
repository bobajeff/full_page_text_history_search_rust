use async_std::task::TaskId;
use futures::channel::mpsc::Sender;
use futures::{SinkExt, StreamExt};
use std::fs;
use std::sync::Arc;

use chromiumoxide::Page;
use chromiumoxide_cdp::cdp::js_protocol::runtime::{AddBindingParams, EventBindingCalled};

use crate::EntryData;

async fn get_eval_string() -> Result<String, Box<dyn std::error::Error>> {
    let return_string = fs::read_to_string("evaluate_script.js")?;
    Ok(return_string)
}

pub async fn page_ops(
    page: Page,
    page_op_id: TaskId,
    mut sender: Sender<(TaskId, EntryData)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = page.url().await?;
    match address {
        Some(address) => {
            let _ = sender.send((page_op_id, EntryData::Address(address))).await;
        }
        None => {}
    }

    let title = page.get_title().await?;
    match title {
        Some(title) => {
            let _ = sender.send((page_op_id, EntryData::Title(title))).await;
        }
        None => {}
    }

    //Extract text from pages
    let _ = page.execute(AddBindingParams::new("addToText")).await;
    let mut events = page.event_listener::<EventBindingCalled>().await?;
    async_std::task::spawn(async move {
        while let Some(event) = events.next().await {
            let v: serde_json::Value = serde_json::from_str(&event.payload).expect("msg");
            let array_of_strings = &v["args"][0].as_array().unwrap();
            let mut text = "".to_string();
            for string in array_of_strings.iter() {
                text += string.as_str().unwrap();
            }
            let _ = sender.send((page_op_id, EntryData::Text(text))).await;
        }
    });

    let eval_script = get_eval_string().await?;

    let _ = page.evaluate(eval_script).await;

    Ok(())
}
