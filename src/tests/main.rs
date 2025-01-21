use std::fs;

use crate::{add_link_to_rss_feed, remove_link_from_rss_feed};

#[test]
fn adds_and_removes_link_properly() {
    let file_location = format!(
        "{}/src/tests/test_links.yml",
        std::env::current_dir().unwrap().display()
    );

    let new_link = "http://test_4.com/".to_string();

    add_link_to_rss_feed(file_location.clone(), Some(new_link.clone()));

    let mut links: Vec<String> = Vec::new();
    let data: Vec<u8> = fs::read(file_location.clone()).unwrap();

    match String::from_utf8(data) {
        Ok(data) => {
            for line in data.lines() {
                links.push(line.to_string());
            }
        }
        _ => {}
    };

    assert_eq!(
        links,
        vec![
            "http://first_link.com/".to_string(),
            "http://second_link.com/".to_string(),
            "http://third_link.com/".to_string(),
            new_link.clone()
        ]
    );

    remove_link_from_rss_feed(file_location.clone(), Some(new_link.clone()));

    let mut links: Vec<String> = Vec::new();
    let data: Vec<u8> = fs::read(file_location.clone()).unwrap();

    match String::from_utf8(data) {
        Ok(data) => {
            for line in data.lines() {
                links.push(line.to_owned());
            }
        }
        _ => {}
    };

    assert_eq!(
        links,
        vec![
            "http://first_link.com/".to_string(),
            "http://second_link.com/".to_string(),
            "http://third_link.com/".to_string(),
        ]
    );
}
