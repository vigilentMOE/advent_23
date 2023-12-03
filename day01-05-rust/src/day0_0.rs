use std::fs::File;
use std::io::{self, BufRead};

pub fn zero_entry() -> io::Result<()> {

    let path = "src/day0_0_input.txt"; 
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line?; 
        lines.push(line);
    }

    for line in &lines {
        println!("{}", line);
    }

    let mut total_sum = 0;

    for line in &lines {
        if let Some(first_digit) = find_first_digit(&line) {
            if let Some(last_digit) = find_last_digit(&line) {
                let combined_value = combine_digits(first_digit, last_digit);
                total_sum += combined_value;
            }
        }
    }

    println!("Total sum of calibration values: {}", total_sum);
    Ok(())
}

pub fn find_first_digit(line: &str) -> Option<u32> {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10);
        }
    }
    None
}

pub fn find_last_digit(line: &str) -> Option<u32> {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10);
        }
    }
    None
}

pub fn combine_digits(first_digit: u32, last_digit: u32) -> u32 {
    let combined = 10 * first_digit + last_digit;
    return combined
}
