use std::collections::HashMap;

pub async fn get_ws_url() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let web_socket_debugger_url = std::thread::spawn(|| {
        //Keep tokio from panicing if run from a existing tokio runtime
        let web_socket_debugger_url = tokio::runtime::Runtime::new() //Keep reqwest from panicking if run without a tokio runtime
            .unwrap()
            .block_on(async { wrap_in_result_function().await.unwrap_or_default() });
        web_socket_debugger_url
    })
    .join()
    .expect("Thread panicked");
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
