use file_utils::config_file;
use reqwest::Url;
use std::env::args;
use std::fs::OpenOptions;
use std::io::Write;

mod displayer;
mod feed_parser;
mod file_utils;
mod iterator;

fn main() {
    let command = args()
        .nth(1)
        .expect("You need to provide input, try help for additional info");
    let options = args().nth(2);

    match command.trim() {
        "rss" => displayer::print_rss_feed(),
        "add" => add_link_to_rss_feed(config_file(), options),
        "remove" => remove_link_from_rss_feed(config_file(), options),
        "help" => displayer::help_menu(),
        "version" => println!("0.1.0"),
        _ => println!("Invalid input"),
    }
}

fn add_link_to_rss_feed(file: String, url: Option<String>) {
    let url = url.expect("no url given");
    let url = Url::parse(&url).expect("invalid url");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file)
        .unwrap();

    writeln!(file, "{url}").expect("unable to write to file");
}

fn remove_link_from_rss_feed(file: String, url: Option<String>) {
    let url = url.expect("no url given");

    match file_utils::read_file(file.clone()) {
        Ok(links) => {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file)
                .unwrap();

            for link in links.into_iter().filter(|link| link != &url) {
                let url = Url::parse(&link).expect("invalid url");
                writeln!(file, "{url}").expect("unable to write to file");
            }
        }
        Err(_) => {}
    }
}

#[cfg(test)]
#[path = "./tests/main.rs"]
mod tests;
