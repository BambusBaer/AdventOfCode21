use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn read_all_lines(file_name: &str) -> Vec<u64>{
    println!("Start reading Sonar Sweeper Data.");
    //let file_name = "sonarSweeperValues.txt";
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);
    let mut result : Vec<u64> = Vec::new();
    let mut line_number = 0;
    
    for line in buf_reader.lines() {
        match line  {
            Ok(_value) => result[line_number] = _value.parse().unwrap(),
            Err(_error) => println!("Couldn't read line {0} of file {1}. {2}", line_number, file_name, _error)
        };
        line_number = line_number + 1;
    }
    
    result
}