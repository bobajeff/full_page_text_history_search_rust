use chrono::prelude::*;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{collector::TopDocs, directory::MmapDirectory};
use tantivy::{doc, Index, IndexWriter, ReloadPolicy};

pub struct Entry {
    timestamp: i64,
    address: &'static str,
    title: &'static str,
    text: &'static str,
}

pub fn index_document(index_writer: &mut IndexWriter, entry: Entry, schema: Schema) -> tantivy::Result<()> 
{
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
    
    index_writer.commit()?;

    Ok(())
}

pub fn init() -> tantivy::Result<()> {
    let index_path = MmapDirectory::open("./data")?; //if data directory doesn't exist; nothing else is run
    let mut schema_builder = Schema::builder();
    schema_builder.add_date_field("timestamp", STORED);
    schema_builder.add_text_field("address", TEXT | STORED);
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let text = schema_builder.add_text_field("text", TEXT);

    let schema = schema_builder.build();
    let index = Index::open_or_create(index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;

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
    //dummy entry
    let _ = index_document(&mut index_writer, entry, schema.clone());


    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![title, text]);
    let query = query_parser.parse_query("too green")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
