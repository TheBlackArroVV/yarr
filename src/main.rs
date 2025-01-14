use std::env::args;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use reqwest::Url;
use crate::file_utils::{config_directory_location, config_file_location};

mod iterator;
mod feed_parser;
mod displayer;
mod file_utils;

fn main() {
    let command = args().nth(1).expect("no command given");
    let options = args().nth(2);

    match command.trim() {
        "rss" => displayer::print_rss_feed(),
        "add" => add_link_to_rss_feed(options),
        _ => println!("Invalid input"),
    }
}

fn add_link_to_rss_feed(url: Option<String>) {
    let url = url.expect("no url given");
    let url = Url::parse(&url).expect("invalid url");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(config_file())
        .unwrap();

    writeln!(file, "{url}").expect("unable to write to file");
}

fn config_file() -> String {
   let file = config_file_location();

    if Path::new(&file).exists() {
        return file;
    }

    fs::create_dir_all(config_directory_location()).unwrap();

    File::create(&file)
        .expect("unable to create file");

    file
}
