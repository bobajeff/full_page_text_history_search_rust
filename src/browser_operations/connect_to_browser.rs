use std::sync::Arc;

use crate::{EntryData, browser_operations::get_ws_url::get_ws_url};
use crate::browser_operations::page_operations::page_ops;
use async_std::task::{self, TaskId};
use futures::channel::mpsc::Sender;
use chromiumoxide::browser::Browser;
use futures::StreamExt;

pub async fn connect_to_browser(mut sender: Sender<(TaskId, EntryData)>) -> Result<(), Box<dyn std::error::Error>> {
    let debug_ws_url = get_ws_url().await?;
    let (browser, mut handler) = Browser::connect(debug_ws_url).await?;

    let handle = async_std::task::spawn(async move {
        loop {
            handler.next().await;
        }
    });

    let mut events = browser.target_changed_listener().await?;
    while let Some(event) = events.next().await {
        if event.target_info.r#type == "page" {
            let target_id = event.target_info.target_id.clone();
            let page = browser.get_page(target_id).await;
            match page {
                Ok(page) => {
                    let sender = sender.clone();
                    async_std::task::spawn(async move {                        
                        let page_op_id = task::current().id();
                        let _ = page_ops(page, page_op_id, sender).await;
                    });
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    }

    handle.await;
    Ok(())
}
