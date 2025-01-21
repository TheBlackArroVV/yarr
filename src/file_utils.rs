use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

use crate::feed_parser::ParserError;

pub fn config_file_location() -> String {
    format!("{}/rss_links.yml", config_directory_location())
}

pub fn config_file() -> String {
    let file = config_file_location();

    if Path::new(&file).exists() {
        return file;
    }

    fs::create_dir_all(config_directory_location()).unwrap();

    File::create(&file).expect("unable to create file");

    file
}

pub fn read_file(file_location: String) -> Result<Vec<String>, Box<dyn Error>> {
    let data: Vec<u8> = fs::read(file_location).unwrap();

    match String::from_utf8(data) {
        Ok(data) => {
            let mut links: Vec<String> = Vec::new();
            for line in data.lines() {
                links.push(line.to_string());
            }
            Ok(links)
        }
        Err(error) => Err(ParserError::from(error)),
    }
}

fn config_directory_location() -> String {
    let home_directory_path = std::env::var("HOME").expect("no home directory");
    format!("{}/.config/yarr", home_directory_path)
}

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
                "{}/src/test_links.yml",
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
