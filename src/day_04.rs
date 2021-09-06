use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

type Password = HashMap<String, String>;

fn has_required_keys(pas: &Password) -> bool {
    // Required fields: byr (Birth Year), iyr (Issue Year), eyr (Expiration Year), hgt (Height),
    // hcl (Hair Color), ecl (Eye Color), pid (Passport ID)
    // Optional field: cid (Country ID)
    pas.contains_key("byr")
        && pas.contains_key("iyr")
        && pas.contains_key("eyr")
        && pas.contains_key("hgt")
        && pas.contains_key("hcl")
        && pas.contains_key("ecl")
        && pas.contains_key("pid")
}

fn as_option(flag: bool) -> Option<()> {
    if flag {
        Some(())
    } else {
        None
    }
}

// byr (Birth Year) - four digits; at least 1920 and at most 2002.
fn is_byr_valid(val: &str) -> Option<()> {
    u16::from_str(&val)
        .ok()
        .and_then(|byr| as_option(byr >= 1920 && byr <= 2002))
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
fn is_iyr_valid(val: &str) -> Option<()> {
    u16::from_str(&val)
        .ok()
        .and_then(|iyr| as_option(iyr >= 2010 && iyr <= 2020))
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
fn is_eyr_valid(val: &str) -> Option<()> {
    u16::from_str(&val)
        .ok()
        .and_then(|eyr| as_option(eyr >= 2020 && eyr <= 2030))
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn is_hgt_valid(val: &str) -> Option<()> {
    if val.ends_with("cm") {
        u16::from_str(&val[..(val.len() - 2)])
            .ok()
            .and_then(|h| as_option(h >= 150 && h <= 193))
    } else if val.ends_with("in") {
        u16::from_str(&val[..(val.len() - 2)])
            .ok()
            .and_then(|h| as_option(h >= 59 && h <= 76))
    } else {
        None
    }
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn is_hcl_valid(val: &str) -> Option<()> {
    if val.starts_with("#") {
        let hcl = &val[1..];
        as_option(hcl.len() == 6 && hcl.chars().all(|c| c.is_digit(16)))
    } else {
        None
    }
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn is_ecl_valid(val: &str) -> Option<()> {
    let cls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    as_option(cls.iter().any(|&c| val == c))
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_pid_valid(val: &str) -> Option<()> {
    as_option(val.len() == 9 && val.chars().all(|c| c.is_digit(10)))
}

fn has_valid_values(pas: &Password) -> bool {
    pas.get("byr")
        .and_then(|val| is_byr_valid(&val))
        .and(pas.get("iyr"))
        .and_then(|val| is_iyr_valid(&val))
        .and(pas.get("eyr"))
        .and_then(|val| is_eyr_valid(&val))
        .and(pas.get("hgt"))
        .and_then(|val| is_hgt_valid(&val))
        .and(pas.get("hcl"))
        .and_then(|val| is_hcl_valid(&val))
        .and(pas.get("ecl"))
        .and_then(|val| is_ecl_valid(&val))
        .and(pas.get("pid"))
        .and_then(|val| is_pid_valid(&val))
        .is_some()
    // cid (Country ID) - ignored, missing or not.
}

fn read_password(mut data: &str) -> Password {
    let mut pas: Password = Password::new();

    // Each passport is represented as a sequence of key:value pairs separated by spaces or newlines.
    while let Some(i) = data.find(':') {
        // Read key.
        let key = data[0..i].trim();
        data = data[(i + 1)..].trim();
        // Read value.
        let value = if let Some(j) = data.find(char::is_whitespace) {
            let v = data[0..j].trim();
            data = data[(j + 1)..].trim();
            v
        } else {
            let v = data.trim();
            data = "";
            v
        };
        pas.insert(key.to_string(), value.to_string());
    }
    pas
}

fn read_input() -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::File::open("./data/day-04.txt")?;

    let mut pas: Vec<String> = Vec::new();
    let mut buf: Option<String> = None;

    for line in BufReader::new(file).lines() {
        if let Ok(s) = line {
            let t = s.trim();
            // Passports are separated by blank lines.
            if t.is_empty() {
                if let Some(w) = buf.take() {
                    pas.push(w);
                }
            } else {
                match buf.as_mut() {
                    Some(w) => {
                        w.push(' ');
                        w.push_str(t);
                    }
                    None => buf = Some(String::from(t)),
                }
            }
        }
    }
    if let Some(w) = buf.take() {
        pas.push(w);
    }

    Ok(pas)
}

pub fn run() {
    println!("--- Day 4: Passport Processing ---");

    // Count the number of valid passports - those that have all required fields.
    let items: Vec<Password> = read_input()
        .expect("invalid input")
        .iter()
        .map(|s| read_password(s))
        .filter(has_required_keys)
        .collect();

    // Your puzzle answer was 239.
    println!(
        "Part 1. The number of passwords with valid keys: {}",
        items.len()
    );

    // --- Part Two ---
    // Count the number of valid passports - those that have all required fields and valid values.
    let count = items.iter().filter(|pas| has_valid_values(&pas)).count();

    // Your puzzle answer was 188.
    println!(
        "Part 2. The number of passwords with valid values: {}",
        count
    );
}
