use crate::iterator::iterator;

pub fn print_rss_feed() {
    println!("RSS Feed Entries:");
    println!("-----------------");

    for entry in iterator() {
        println!("Title: {}", entry.title);
        println!("Link: {}", entry.link);
        println!("Updated: {}", entry.updated);
        println!("Author: {}", entry.author.name);
        println!();
    }
}

pub fn help_menu() {
    println!("Help menu");
    println!("-----------------");
    println!("Available commands:");
    println!("add _url_");
    println!("remove _url_");
    println!("help");
    println!("Thanks for using my rss reader!");
}
