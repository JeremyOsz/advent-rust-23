use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// ReadInput reads calibrationValues from ./input.txt
fn read_input() -> io::Result<Vec<String>> {
    let path = Path::new("./dec_1/input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().filter_map(io::Result::ok).collect())
}

pub fn run() -> io::Result<()> {
    let input = read_input()?;

    let mut sum = 0;
    for line in &input {
        match get_first_and_last_number(&line) {
            Ok(number) => sum += number,
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    println!("{}", sum);
    Ok(())
}

// parse_number maps number words to numbers
fn parse_number(number: &str) -> Option<char> {
    let numbers_to_string = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    if let Some(&digit) = numbers_to_string.get(number) {
        Some(digit)
    } else {
        number.parse::<char>().ok()
    }
}

// reverse reverses a string
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// get_first_and_last_number searches a string for a number character from the start and end of the string
fn get_first_and_last_number(s: &str) -> Result<i32, &'static str> {
    let re = regex::Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let first_number = re.find(s).ok_or("No first number found")?.as_str();


    let reverse_s = reverse(s);
    let reverse_re =
        regex::Regex::new(r"[0-9]|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let last_number = reverse_re
        .find(&reverse_s)
        .ok_or("No last number found")?
        .as_str();

    // Print the last number

    let first_digit = parse_number(first_number).ok_or("Failed to parse first number")?;
    let last_digit_restored = reverse(last_number);
    let last_digit = parse_number(&last_digit_restored).ok_or("Failed to parse last number")?;

    // combine the first and last number as strings
    let combined_number = format!("{}{}", first_digit, last_digit);

    // parse the combined number as an i32
    let combined_number = combined_number.parse::<i32>().unwrap();

    Ok(combined_number)
}
