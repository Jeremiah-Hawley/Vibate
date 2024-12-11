// this file wil be called from main and it will pull settings from the settings.config file
// going to do this first so i can get a hang of rust again, it's been a while :(
// settings.config
//use std::env::current_dir; // for interacting with the enviornment the code runs in
use ::std::io::{self, BufRead}; // for reading into buffer to save memory on large files
use ::std::path::Path;
use std::fs::File; //for opening file //for checking that FILE_PATH is valid

const FILE_PATH: &str = "./.default.config";

pub fn read_config() {
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines.flatten() {
            println! {"{}", line}; // here we can fuck with line data
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
