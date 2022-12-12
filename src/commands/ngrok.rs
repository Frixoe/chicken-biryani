use std::fs;
use std::io::prelude::*;
use home;

pub fn set_ngrok(args: &Vec<String>) {
    // Path value is at index 2 
    let filepath = &args[2];

    // Get home dir 
    match home::home_dir() {
        Some(path) => {
            let write_path: &String = &format!("{}/.chicken-biryani/", path.display());

            // Making the directory
            fs::DirBuilder::new()
                .recursive(true)
                .create(write_path).unwrap();

            // Create the config.toml
            let file = fs::File::create(format!("{}config.toml", write_path));

            // Write to the file
            _ = file.expect("Couldn't open file").write_all(format!("ngrok = \"{}\"", filepath).as_bytes());
            
            println!("ngrok path has been set!");
        },
        None => println!("Couldn't get your home directory!"),
    }
}
