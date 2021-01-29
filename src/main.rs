use full_page_text_history_search::{connect_to_browser, init, Entry};

use chrono::prelude::*;
use futures::{channel::mpsc::channel, SinkExt};
use std::sync::Arc;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel(1);

    let cdp_client_process = async_std::task::spawn(async move {
        connect_to_browser(tx).await;
    });

    let index_values = init(rx).await;

    let (index_values, listener_handle) = index_values.unwrap();

    let _ = full_page_text_history_search::index_management::test_index(index_values);

    cdp_client_process.await;
    listener_handle.await;
    Ok(())
}
