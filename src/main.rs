use full_page_text_history_search::connect_to_browser;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    connect_to_browser().await?;
    Ok(())
}
