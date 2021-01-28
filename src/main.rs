// use full_page_text_history_search::connect_to_browser;
use full_page_text_history_search::init;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect_to_browser().await?;
    let _ = init();

    Ok(())
}
