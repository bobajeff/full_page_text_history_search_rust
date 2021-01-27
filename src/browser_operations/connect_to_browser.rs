use crate::browser_operations::get_ws_url::get_ws_url;
use crate::browser_operations::page_operations::page_ops;
use chromiumoxide::browser::Browser;
use futures::StreamExt;

pub async fn connect_to_browser() -> Result<(), Box<dyn std::error::Error>> {
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
            let page = browser.get_page(event.target_info.target_id.clone()).await;
            match page {
                Ok(page) => {
                    async_std::task::spawn(async move {
                        let _ = page_ops(page).await;
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
