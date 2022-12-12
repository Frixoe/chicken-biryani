use std::fs;
use std::io::prelude::*;
use home;

pub fn set_ngrok(args: &Vec<String>) {
    // Path value is at index 2
    let filepath = &args[2];

    // Get home dir
    match home::home_dir() {
        Some(path) => {
            let write_path = format!("{}/.chicken-biryani/", path.display());

            // Making the directory
            fs::create_dir_all(&write_path).unwrap();

            // Create the config.toml
            let mut file = fs::File::create(format!("{}config.toml", write_path)).unwrap();

            // Write to the file
            file.write_all(format!("ngrok = \"{}\"", filepath).as_bytes()).unwrap();

            println!("ngrok path has been set!");
        },
        None => println!("Couldn't get your home directory!"),
    }
}
