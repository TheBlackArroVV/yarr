pub fn config_file_location() -> String {
    format!("{}/rss_links.yml", config_directory_location())
}

pub fn config_directory_location() -> String {
    let home_directory_path = std::env::var("HOME").expect("no home directory");
    format!("{}/.config/yarr", home_directory_path)
}
