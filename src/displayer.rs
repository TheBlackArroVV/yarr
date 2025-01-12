use crate::iterator;

pub fn print_rss_feed() {
    println!("RSS Feed Entries:");
    println!("-----------------");

    for entry in iterator::iterator() {
        println!("Title: {}", entry.title);
        println!("Link: {}", entry.link);
        println!("Updated: {}", entry.updated);
        println!("Author: {}", entry.author.name);
        println!();
    }
}
