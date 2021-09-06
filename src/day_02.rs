use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

// Each line gives the password policy and then the password.
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
struct Password {
    start: usize,
    end: usize,
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule, password) = parse_pair(s, ":").expect("rule/password separation fail");
        let (range, policy) = parse_pair(rule, " ").expect("range/policy separation fail");
        let (start, end) = parse_pair(range, "-").expect("range start/end separation fail");

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
            letter: policy.chars().next().expect("empty policy"),
            password: password.to_string(),
        })
    }
}

fn read_input() -> Result<Vec<Password>, Box<dyn Error>> {
    let file = File::open("./data/day-02.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        buf.push(line?.parse()?);
    }
    Ok(buf)
}

pub fn run() {
    println!("--- Day 2: Password Philosophy ---");

    let passwords = read_input().expect("fail to read expense report file");

    // The password policy indicates the lowest and highest number of times a given letter must
    // appear for the password to be valid.
    // How many passwords are valid according to their policies?
    let count = passwords
        .iter()
        .filter(|pas| {
            let x = pas.password.chars().filter(|&ch| ch == pas.letter).count();
            x >= pas.start && x <= pas.end
        })
        .count();

    // Your puzzle answer was 572.
    println!("Part 1. The number of valid passwords: {}", count);

    // --- Part Two ---

    // Each policy actually describes two positions in the password, where 1 means the first
    // character, 2 means the second character, and so on.
    // (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
    // Exactly one of these positions must contain the given letter. Other occurrences of the
    // letter are irrelevant for the purposes of policy enforcement.
    let count = passwords
        .iter()
        .filter(|pas| {
            pas.password
                .chars()
                .enumerate()
                .filter(|(i, ch)| (i + 1 == pas.start || i + 1 == pas.end) && *ch == pas.letter)
                .count()
                == 1
        })
        .count();
    // Your puzzle answer was 306.
    println!("Part 2. The number of valid passwords: {}", count);
}
