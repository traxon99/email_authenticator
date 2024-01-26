use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_pass_from_file(file_dir: &str) -> String{
    let path = Path::new(file_dir);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => return s,
    }    
}
