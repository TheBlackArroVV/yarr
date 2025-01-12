use crate::feed_parser;
use crate::feed_parser::Entry;

const TEST_RSS_LINKS_ARRAY: [&str; 2] = [
    "https://world.hey.com/dhh/feed.atom", 
    "https://martinfowler.com/feed.atom"
];

pub fn iterator() -> Vec<Entry> {
    let mut all_entries: Vec<Entry> = Vec::new();

    for link in TEST_RSS_LINKS_ARRAY.iter() {
        match feed_parser::feed_parser(link) {
            Ok(entries) => {
                for entry in entries {
                    all_entries.push(entry);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }

    sort_entries(all_entries)
}

fn sort_entries(mut entries: Vec<Entry>) -> Vec<Entry> {
    entries.sort_by(|a, b| a.updated.cmp(&b.updated));
    entries
}
