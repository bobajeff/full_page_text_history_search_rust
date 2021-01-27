use chromiumoxide::Page;

pub async fn page_ops(page: Page) -> Result<(), Box<dyn std::error::Error>> {
    let title = page.get_title().await?;
    match title {
        Some(title) => {
            println!("{}", title);
        }
        None => {}
    }
    Ok(())
}
