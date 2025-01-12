use std::env::args;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use reqwest::Url;

mod iterator;
mod feed_parser;
mod displayer;

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
    let home_directory_path = std::env::var("HOME").expect("no home directory");
    let config_directory = format!("{}/.config/yarr", home_directory_path);
    let file_directory = format!("{}/rss_links.yml", config_directory);

    if Path::new(&file_directory).exists() {
        return file_directory;
    }

    fs::create_dir_all(&config_directory).unwrap();

    File::create(&file_directory)
        .expect("unable to create file");

    file_directory
}
