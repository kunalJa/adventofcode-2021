
use std::fs;

fn main() {
    let input_path = "inputs/01.in";
    let input = fs::read_to_string(input_path).expect("Unable to read from file");
    let mut prev = std::i32::MAX;
    let mut count = 0;
    for line in input.split('\n') {
        let current = line.trim().parse::<i32>().unwrap();
        if current > prev {
            count += 1;
        }
        prev = current;
    }
    
    println!("Result: {}", count);
}