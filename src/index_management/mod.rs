use std::println;

use futures::StreamExt;
use futures::channel::mpsc::Receiver;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{collector::TopDocs, directory::MmapDirectory};
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

use crate::EntryStream;

use super::Entry;

pub struct IndexerValues {
    timestamp: Field,
    address: Field,
    title: Field,
    text: Field,
    schema: Schema,
    index: Index
}


pub async fn index_document(
    index_writer: &mut IndexWriter,
    entry: Entry,
    schema: Schema,
) -> tantivy::Result<()> {
    let timestamp = schema.get_field("timestamp").unwrap();
    let address = schema.get_field("address").unwrap();
    let title = schema.get_field("title").unwrap();
    let text = schema.get_field("text").unwrap();

    index_writer.add_document(doc!(
    timestamp => entry.timestamp,
    address => entry.address,
    title => entry.title,
    text => entry.text
    ));

    let result = index_writer.commit(); //this doesn't block
    
    Ok(())
}

pub fn test_index(index_values: IndexerValues) -> tantivy::Result<()> {
    let reader = index_values.index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index_values.index, vec![index_values.title, index_values.text]);
    let query = query_parser.parse_query("too green")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", index_values.schema.to_json(&retrieved_doc));
    }

    Ok(())
}

pub async fn init(rx: Receiver<Entry>) -> tantivy::Result<IndexerValues> {
    let mut entries = EntryStream::new(rx);
    let index_path = MmapDirectory::open("./data")?; //if data directory doesn't exist; nothing else is run
    let mut schema_builder = Schema::builder();
    let timestamp = schema_builder.add_date_field("timestamp", STORED);
    let address = schema_builder.add_text_field("address", TEXT | STORED);
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let text = schema_builder.add_text_field("text", TEXT);

    let schema = schema_builder.build();
    let index = Index::open_or_create(index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;

    let index_values = IndexerValues { timestamp, address, title, text, schema: schema.clone(), index };

    async_std::task::spawn(async move {
        while let Some(entry) = entries.next().await {
            index_document(&mut index_writer, entry, schema.clone()).await;
        }
    });
    // test_index(index, title, text, schema.clone())?;

    Ok(index_values)
}
