use std::collections::HashMap;
use chromiumoxide::browser::Browser;
use futures::StreamExt;

pub async fn get_ws_url() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let web_socket_debugger_url = std::thread::spawn(|| {//Keep tokio from panicing if run from a existing tokio runtime
        let web_socket_debugger_url = tokio::runtime::Runtime::new() //Keep reqwest from panicking if run without a tokio runtime
            .unwrap()
            .block_on(async { wrap_in_result_function().await.unwrap_or_default() });
            web_socket_debugger_url
    }).join().expect("Thread panicked");
    Ok(web_socket_debugger_url)
}

async fn wrap_in_result_function() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://127.0.0.1:9222/json/version")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let web_socket_debugger_url = resp["webSocketDebuggerUrl"].clone();
    Ok(web_socket_debugger_url)
}

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
                        }
                        None => {}
                    }
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