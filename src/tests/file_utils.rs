#[cfg(test)]
mod tests {
    use crate::file_utils::{config_file_location, read_file};

    #[test]
    fn config_file_location_is_correct() {
        let home_directory_path = std::env::var("HOME").expect("no home directory");
        assert_eq!(
            config_file_location(),
            format!("{}/.config/yarr/rss_links.yml", home_directory_path)
        )
    }

    #[test]
    fn links_are_properly_parsed() {
        assert_eq!(
            read_file(format!(
                "{}/src/tests/test_links.yml",
                std::env::current_dir().unwrap().display()
            ))
            .unwrap(),
            vec![
                "http://first_link.com/".to_string(),
                "http://second_link.com/".to_string(),
                "http://third_link.com/".to_string(),
            ]
        );
    }
}
