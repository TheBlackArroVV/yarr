mod feed_parser;

fn main() {
    match feed_parser::main::main() {
        Ok(entries) => {
            for entry in entries {
                println!("Title: {}", entry.title);
                println!("Link: {}", entry.link);
                println!("Updated: {}", entry.updated);
                println!("Author: {}", entry.author.name);
                println!();
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}
