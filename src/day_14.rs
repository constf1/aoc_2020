use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

struct Task {
    mask: String,
    data: Vec<(usize, usize)>,
}

impl Task {
    pub fn new(mask: &str) -> Self {
        Self {
            mask: mask.to_string(),
            data: Vec::new(),
        }
    }
}

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

fn read_input() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = fs::File::open("./data/day-14.txt")?;

    let mut buf = Vec::new();
    let mut task: Option<Task> = None;
    for line in BufReader::new(file).lines() {
        let s = line?;
        let pair = parse_pair(&s, "=").ok_or("fail to parse")?;
        if pair.0 == "mask" {
            if let Some(t) = task.take() {
                buf.push(t);
            }
            task = Some(Task::new(pair.1));
        } else if pair.0.starts_with("mem") {
            let adr = pair
                .0
                .find('[')
                .zip(pair.0.find(']'))
                .map(|x| &pair.0[1 + x.0..x.1])
                .ok_or("fail to parse memory address")?
                .parse()?;
            let val = pair.1.parse()?;

            task.as_mut().expect("mask not found").data.push((adr, val));
        }
    }
    if let Some(t) = task.take() {
        buf.push(t);
    }

    Ok(buf)
}

fn decode(mask: &str, addr: usize) -> Vec<usize> {
    let mut arr = vec![addr];
    for (i, m) in mask.chars().rev().enumerate() {
        // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
        if m == 'X' || m == '1' {
            let one = 1 << i;
            let len = arr.len();

            for i in 0..len {
                let a = arr[i];
                // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
                arr[i] = a | one;

                // If the bitmask bit is X, the corresponding memory address bit is floating.
                if m == 'X' {
                    arr.push(a & !one);
                }
            }
        }
    }
    arr
}

pub fn run() {
    println!("--- Day 14: Docking Data ---");

    let input = read_input().expect("invalid input");

    // Execute the initialization program.
    let mut memo: HashMap<usize, usize> = HashMap::new();
    for task in &input {
        for line in &task.data {
            let mut val = *memo.get(&line.0).unwrap_or(&0);
            for (i, m) in task.mask.chars().rev().enumerate() {
                let one = 1 << i;
                match m {
                    'X' => {
                        val &= !one;
                        val |= line.1 & one;
                    }
                    '0' => val &= !one,
                    '1' => val |= one,
                    _ => (),
                }
            }
            memo.insert(line.0, val);
        }
    }

    // What is the sum of all values left in memory after it completes?
    let n: usize = memo.values().sum();

    // Your puzzle answer was 15172047086292.
    println!("Part 1. The sum of all values left in memory: {}", n);

    // --- Part Two ---

    // Execute the initialization program using an emulator for a version 2 decoder chip.
    memo.clear();
    for task in &input {
        for line in &task.data {
            // A version 2 decoder chip doesn't modify the values being written at all.
            // Instead, it acts as a memory address decoder.
            for addr in decode(&task.mask, line.0) {
                memo.insert(addr, line.1);
            }
        }
    }

    let n: usize = memo.values().sum();

    // Your puzzle answer was 4197941339968.
    println!("Part 2. The sum of all values left in memory: {}", n);
}
