use full_page_text_history_search::get_ws_url;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let debug_ws_url = get_ws_url().await?;
    println!("{}", debug_ws_url);
    Ok(())
}
