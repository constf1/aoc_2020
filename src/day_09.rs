use std::fs;
use std::io::{BufRead, BufReader};

fn read_input() -> std::io::Result<Vec<i64>> {
    let file = fs::File::open("./data/day-09.txt")?;

    let mut buf = Vec::new();

    for line in BufReader::new(file).lines() {
        if let Some(n) = line.ok().and_then(|s| s.trim().parse().ok()) {
            buf.push(n);
        }
    }

    Ok(buf)
}

fn is_valid(preamble: &[i64], value: i64) -> bool {
    for (i, a) in preamble.iter().enumerate().skip(1) {
        for b in preamble[0..i].iter() {
            if a + b == value {
                return true;
            }
        }
    }
    false
}

fn find_invalid_item(items: &[i64], preamble_size: usize) -> Option<i64> {
    for (i, &value) in items.iter().enumerate().skip(preamble_size) {
        if !is_valid(&items[i - preamble_size..i], value) {
            return Some(value);
        }
    }
    None
}

pub fn run() {
    println!("--- Day 9: Encoding Error ---");

    let items = read_input().expect("invalid input");

    // Find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it.
    // What is the first number that does not have this property?
    let preamble_size = 25;
    let invalid_number = find_invalid_item(&items, preamble_size).expect("invalid items");

    // Your puzzle answer was 21806024.
    println!("Part 1. The first invalid number: {}", invalid_number);

    // --- Part Two ---
    // Find a contiguous set of at least two numbers in your list which sum to the invalid number.
    'outer_loop: for start in 0..items.len() {
        let mut sum = items[start];
        for next in start + 1..items.len() {
            sum = sum + items[next];
            if sum == invalid_number {
                // To find the encryption weakness, add together the smallest and largest number in this contiguous range.
                let smallest = items[start..=next].iter().min().unwrap();
                let largest = items[start..=next].iter().max().unwrap();

                // Your puzzle answer was 2986195.
                println!("Part 2. The encryption weakness: {}", smallest + largest);
                break 'outer_loop;
            } else if sum > invalid_number {
                break;
            }
        }
    }
}
