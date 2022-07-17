use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let INPUT_PATH = "inputs/01.in";
    let input_f = File::open(INPUT_PATH)?;
    let mut reader = BufReader::new(input_f);
    let nums: Result<Vec<u32>, Error> = reader.lines().map(
        |l| l.and_then(
            |v| v.parse().map_err(
                |e| Error::new(ErrorKind::InvalidData, e)))).collect();
    
    let nums = nums?;
    let mut count = 0;
    let window_size = 3;
    let mut prev = std::u32::MAX;
    let mut current = 0;

    for i in 0..nums.len() {
        current += nums[i];
        if i >= window_size {
            current -= nums[i - window_size];
            if current > prev {
                count += 1;
            }
        }
        prev = current;
    }

    println!("Result: {}", count);

    
    Ok(())
}