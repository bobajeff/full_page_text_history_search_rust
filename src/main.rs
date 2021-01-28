use full_page_text_history_search::{connect_to_browser, init, Entry};

use futures::{SinkExt, channel::mpsc::channel};
use chrono::prelude::*;
use std::sync::Arc;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel(1);
    
    // connect_to_browser().await?;

    let index_values = init(rx).await;
        //dummy entry
    let entry = Entry {
        timestamp: Local::now().timestamp(),
        address: "http://www.example.com",
        title: "Of Mice and Men",
        text: "A few miles south of Soledad, the Salinas River drops in close to the hillside \
    bank and runs deep and green. The water is warm too, for it has slipped twinkling \
    over the yellow sands in the sunlight before reaching the narrow pool. On one \
    side of the river the golden foothill slopes curve up to the strong and rocky \
    Gabilan Mountains, but on the valley side the water is lined with trees—willows \
    fresh and green with every spring, carrying in their lower leaf junctures the \
    debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
    limbs and branches that arch over the pool",
    };
    let entry_b = Entry {
        timestamp: Local::now().timestamp(),
        address: "http://www.example.com",
        title: "Of Mice and Men",
        text: "A few miles south of Soledad, the Salinas River drops in close to the hillside \
    bank and runs deep and green. The water is warm too, for it has slipped twinkling \
    over the yellow sands in the sunlight before reaching the narrow pool. On one \
    side of the river the golden foothill slopes curve up to the strong and rocky \
    Gabilan Mountains, but on the valley side the water is lined with trees—willows \
    fresh and green with every spring, carrying in their lower leaf junctures the \
    debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
    limbs and branches that arch over the pool",
    };

    let mut tx = tx;
    tx.send(entry).await?;
    tx.send(entry_b).await?;
    
    //sleep for 5 seconds so that IndexWriter::commit() gets a chance to write to the index before running test_index() 
    async_std::task::sleep(std::time::Duration::from_secs(5)).await; 

    let (index_values, listener_handle) = index_values.unwrap();

    let _ = full_page_text_history_search::index_management::test_index(index_values);
    
    listener_handle.await;
    Ok(())
}