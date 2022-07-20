
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let input_path = "inputs/02.in";
    let input_f = File::open(input_path)?;
    let reader = BufReader::new(input_f);
    
    let mut depth = 0;
    let mut horizontal_pos = 0;
    for l in reader.lines() {
        let l = l?;
        let args: Vec<&str> = l.split_whitespace().collect();
        match args[0] {
            "up" => depth -= args[1].parse().unwrap_or(0),
            "down" => depth += args[1].parse().unwrap_or(0),
            "forward" => horizontal_pos += args[1].parse().unwrap_or(0),
            _ => println!("other")
        };
    }
    
    println!("Result: {} x {} = {}", depth, horizontal_pos, depth * horizontal_pos);

    Ok(())
}