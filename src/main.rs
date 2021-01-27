use full_page_text_history_search::get_ws_url;

use futures::StreamExt;
use chromiumoxide::browser::Browser;


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
                    let title = page.get_title().await?;
                    match title {
                        Some(title) => {
                            println!("{}", title);
                        },
                        None => {},
                    }
                },
                Err(e) => {println!("{}", e)}
            }
            
        }
    }

    handle.await;
    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_to_browser().await?;
    Ok(())
}
