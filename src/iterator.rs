use crate::feed_parser::Entry;
use crate::{feed_parser, file_utils};

pub fn iterator() -> Vec<Entry> {
    let mut all_entries: Vec<Entry> = Vec::new();

    match file_utils::read_file(file_utils::config_file_location()) {
        Ok(links) => {
            for link in links {
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
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

    sort_entries(all_entries)
}

fn sort_entries(mut entries: Vec<Entry>) -> Vec<Entry> {
    entries.sort_by(|a, b| a.updated.cmp(&b.updated));
    entries
}

#[cfg(test)]
#[path = "./tests/iterator.rs"]
mod tests;
