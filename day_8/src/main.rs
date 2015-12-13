use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
	let mut total_code_chars = 0;
	let mut total_string_chars = 0;
    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	let s = line.unwrap();

    	let mut curr_code_chars = s.len();
    	let mut curr_string_chars = s.len() - 2;

    	let mut vec_chars = s.chars().collect();

    	for (i, c) in vec_chars.iter().enumerate() {
    	    if c == '\\' {
    	        if i + 1 < vec_chars.num() {
    	            match vec_chars[i+1] {
    	                'x' => curr_string_chars -= 3,
    	                '\"' => curr_string_chars -= 1,
    	                '\\' => curr_string_chars -= 1,
    	                _ => {}
    	            }
    	        }
    	    }
    	}

 		println!("{} -> num_chars = {} num_str = {}", s, curr_code_chars, curr_string_chars);

    	total_code_chars += curr_code_chars;
    	total_string_chars += curr_string_chars;
    }

    println!("total_code_chars = {} total_string_chars = {} total = {}", total_code_chars, total_string_chars, total_code_chars - total_string_chars);
}