use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

impl Instruction {
    fn from(s: &str) -> Option<Self> {
        // Each key: value pairs are separated by space.
        let splitter = " ";
        parse_pair(s, splitter).and_then(|(op, ar)| {
            let argument = ar.parse::<i64>().expect("number parsing fail");
            match op {
                "acc" => Some(Self {
                    operation: Operation::Acc,
                    argument,
                }),
                "jmp" => Some(Self {
                    operation: Operation::Jmp,
                    argument,
                }),
                "nop" => Some(Self {
                    operation: Operation::Nop,
                    argument,
                }),
                _ => None,
            }
        })
    }
}

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

fn read_input() -> std::io::Result<Vec<Instruction>> {
    let file = fs::File::open("./data/day-08.txt")?;

    let mut buf = Vec::new();

    for line in BufReader::new(file).lines() {
        if let Some(inst) = line.ok().and_then(|s| Instruction::from(&s)) {
            buf.push(inst);
        }
    }

    Ok(buf)
}

fn execute(items: &[Instruction]) -> (i64, i64) {
    // The accumulator starts at 0.
    let mut accum: i64 = 0;
    let mut index: i64 = 0;

    // Immediately before any instruction is executed a second time, what value is in the accumulator?
    let mut done: HashSet<i64> = HashSet::new();
    loop {
        if index < 0 || items.len() <= index as usize || done.contains(&index) {
            return (accum, index);
        }
        done.insert(index);

        let inst = &items[index as usize];
        match inst.operation {
            // nop stands for No OPeration - it does nothing.
            // The instruction immediately below it is executed next.
            Operation::Nop => index = index + 1,
            // acc increases or decreases a single global accumulator by the value given in the argument.
            // After an acc instruction, the instruction immediately below it is executed next.
            Operation::Acc => {
                accum = accum + inst.argument;
                index = index + 1
            }
            // jmp jumps to a new instruction relative to itself.
            // The next instruction to execute is found using the argument as an offset from the jmp instruction.
            Operation::Jmp => index = index + inst.argument,
        }
    }
}

pub fn run() {
    println!("--- Day 8: Handheld Halting ---");

    // Run your copy of the boot code.
    // Immediately before any instruction is executed a second time, what value is in the accumulator?

    let items = read_input().expect("invalid input");

    let (accum, _) = execute(&items);

    // Your puzzle answer was 1928.
    println!("Part 1. The accumulator value: {}", accum);

    // --- Part Two ---

    // The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file.
    // By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
    // What is the value of the accumulator after the program terminates?

    for (i, inst) in items.iter().enumerate() {
        if inst.operation != Operation::Acc {
            // Exactly one instruction is corrupted.
            // Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp).
            let mut fix = items.clone();
            fix[i] = Instruction {
                operation: match inst.operation {
                    Operation::Jmp => Operation::Nop,
                    Operation::Nop => Operation::Jmp,
                    op => op,
                },
                argument: inst.argument,
            };

            let (accum, index) = execute(&fix);
            if items.len() == index as usize {
                // Your puzzle answer was 1319.
                println!("Part 2. The fixed accumulator value: {}", accum);
                break;
            }
        }
    }
}
