//command line (cargo run -- read {filename})
//read -h or --help for help

use std::fs;
use std::path::Path;
use std::io::{Read};

//Read file
pub fn reading(file_name: &str) {
    //Find the 'files' folder
    let folder = "files";
    let file_path = Path::new(folder).join(file_name);

    //Error check for file
    if !file_path.exists() {
        println!("The file {} does not exist!", file_name);
        return;
    }

    //Once file located, read it
    match fs::File::open(&file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
			//read_to_string in other words puts the text in the file
			//into a new string(contents)
            if let Err(e) = file.read_to_string(&mut contents) {
                println!("Cannot read the file! {}", e);
            } else {
                // Print the file contents
                println!("Loading file:\n{}", contents);
            }
        }
        Err(e) => {
            println!("Cannot read the file! {}", e);
        }
    }
}
