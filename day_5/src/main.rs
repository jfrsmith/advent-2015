use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
	let mut niceStrings = 0;
    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let nextStr = line.unwrap();
        let mut vowelCount = 0;
        let mut consecutive = false;

        
    }

    println!("Nice strings: {}", niceStrings);
}