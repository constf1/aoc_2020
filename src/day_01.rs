use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input() -> Result<Vec<usize>, Box<dyn Error>> {
    let file = File::open("./data/day-01.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        buf.push(line?.parse()?);
    }
    Ok(buf)
}

pub fn run() {
    println!("--- Day 1: Report Repair ---");

    // Find the two entries that sum to 2020 and then multiply those two numbers together.
    let expense_report = read_input().expect("fail to read expense report file");

    for i in 1..expense_report.len() {
        let entry_1 = expense_report[i];
        for j in 0..i {
            let entry_2 = expense_report[j];
            if entry_1 + entry_2 == 2020 {
                // Your puzzle answer was 1019371.
                println!("Part 1. The entry product: {}", entry_1 * entry_2);
            }
        }
    }

    // --- Part Two ---

    // What is the product of the three entries that sum to 2020?
    for i in 2..expense_report.len() {
        let entry_1 = expense_report[i];
        for j in 1..i {
            let entry_2 = expense_report[j];
            for k in 0..j {
                let entry_3 = expense_report[k];
                if entry_1 + entry_2 + entry_3 == 2020 {
                    // Your puzzle answer was 278064990.
                    println!("Part 2. The entry product: {}", entry_1 * entry_2 * entry_3);
                }
            }
        }
    }
}
