use std::io::{BufReader,BufRead};
use std::fs::File;

enum Command {
    Toggle,
    TurnOff,
    TurnOn,
}

struct Instruction {
    start: (i32, i32),
    end: (i32, i32), 
    command: Command
}

fn parse_instruction(raw_instruction: &str) -> Instruction {
    let mut cmd = Command::Toggle;
	let mut dims_start = Vec::new();
	let mut dims_end = Vec::new();

	let v: Vec<_> = raw_instruction.split_whitespace().collect();
	if v[0] == "turn" {
	    cmd = if v[1] == "off" { Command::TurnOff } else { Command::TurnOn };
	    dims_start = v[2].split(",").collect();
	    dims_end = v[4].split(",").collect();
	}
	else {
	    dims_start = v[1].split(",").collect();
	    dims_end = v[3].split(",").collect();
	}

	Instruction { 	start: (dims_start[0].parse().unwrap(), dims_start[1].parse().unwrap()), 
					end: (dims_end[0].parse().unwrap(), dims_end[1].parse().unwrap()), 
					command: cmd  }
}

fn modify_lights(light_map: &[bool], do_instruction: Instruction) -> () {
    // add code here
}

fn main() {
	let mut lights: [bool; 1000*1000] = [false; 1000 * 1000];

    let file = File::open("input/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
    	modify_lights(&lights, parse_instruction(&line.unwrap()));
    }

    let mut num_on_lights = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if lights[(x * 1000) + y] {
                num_on_lights += 1;
            }
        }
    }

    println!("num_on_lights = {}", num_on_lights);
}