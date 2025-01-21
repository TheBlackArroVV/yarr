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
mod tests {
    use crate::{
        feed_parser::{Author, Entry},
        iterator::sort_entries,
    };

    #[test]
    fn sorts_by_updated() {
        let entries: Vec<Entry> = vec![
            Entry {
                title: "a".to_string(),
                link: "A".to_string(),
                updated: "19.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
            Entry {
                title: "b".to_string(),
                link: "A".to_string(),
                updated: "13.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
            Entry {
                title: "c".to_string(),
                link: "A".to_string(),
                updated: "22.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
        ];

        let sorted_entries: Vec<Entry> = vec![
            Entry {
                title: "b".to_string(),
                link: "A".to_string(),
                updated: "13.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
            Entry {
                title: "a".to_string(),
                link: "A".to_string(),
                updated: "19.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
            Entry {
                title: "c".to_string(),
                link: "A".to_string(),
                updated: "22.01.2025".to_string(),
                author: Author {
                    name: "A".to_string(),
                },
            },
        ];

        assert_eq!(sort_entries(entries), sorted_entries);
    }
}
