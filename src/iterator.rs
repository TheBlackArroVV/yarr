use std::error::Error;
use std::fs;
use crate::{feed_parser, file_utils};
use crate::feed_parser::{Entry, ParserError};

pub fn iterator() -> Vec<Entry> {
    let mut all_entries: Vec<Entry> = Vec::new();

    match links() {
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

fn links() -> Result<Vec<String>, Box<dyn Error>> {
    let file = file_utils::config_file_location();

    let data: Vec<u8> = fs::read(file).unwrap();

    match String::from_utf8(data) {
        Ok(data) => {
            let mut links: Vec<String> = Vec::new();
            for line in data.lines() {
                links.push(line.to_string());
            }
            Ok(links)
        }
        Err(error) => {
            Err(ParserError::from(error))
        }
    }
}
