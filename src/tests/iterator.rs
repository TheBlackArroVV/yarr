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
