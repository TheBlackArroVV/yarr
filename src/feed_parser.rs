use quick_xml::Reader;
use quick_xml::events::Event;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub title: String,
    pub link: String,
    pub updated: String,
    pub author: Author
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub name: String
}

pub type ParserError = Box<dyn Error>;

#[tokio::main]
pub async fn feed_parser(url: String) -> Result<Vec<Entry>, ParserError> {
    match read_feed(url).await {
        Ok(entries) => Ok(entries),
        Err(error) => Err(error),
    }
}

fn empty_entry() -> Entry {
    Entry {
        title: String::new(),
        link: String::new(),
        updated: String::new(),
        author: Author {
            name: String::new()
        },
    }
}

async fn read_feed(url: String) -> Result<Vec<Entry>, ParserError> {
    let response = reqwest::get(url).await?.text().await?;
    let mut reader = Reader::from_str(&response);

    reader.trim_text(true);

    let mut entries = Vec::new();

    match parse_entries(&mut reader, &mut entries) {
        Some(value) => value,
        None => Ok(entries),
    }
}

fn parse_entries(reader: &mut Reader<&[u8]>, entries: &mut Vec<Entry>) -> Option<Result<Vec<Entry>, ParserError>> {
    let mut current_entry = empty_entry();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"title" => {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Text(e)) => {
                            current_entry.title = e.unescape_and_decode(&reader).unwrap();
                        }
                        _ => (),
                    }
                }
                b"updated" => {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Text(e)) => {
                            current_entry.updated = e.unescape_and_decode(&reader).unwrap();
                        },
                        _ => (),
                    }
                }
                b"author" => {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => match e.name() {
                            b"name" => {
                                match reader.read_event(&mut buf) {
                                    Ok(Event::Text(e)) => {
                                        current_entry.author.name = e.unescape_and_decode(&reader).unwrap();
                                    },
                                    _ => (),
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => {
                if e.name() == b"entry" {
                    entries.push(current_entry);
                    current_entry = empty_entry();
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Empty(ref e)) => {
                if e.name() == b"link" {
                    match e.attributes().with_checks(false).find(|a| a.as_ref().unwrap().key == b"href") {
                        Some(attr) => {
                            current_entry.link = attr.unwrap().unescape_and_decode_value(&reader).unwrap();
                        },
                        _ => (),
                    }
                }
            },
            Err(e) => return Some(Err(Box::new(e))),
            _ => (),
        }
        buf.clear();
    }
    None
}
