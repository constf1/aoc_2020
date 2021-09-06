use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

type Group = Vec<String>;

fn read_input() -> std::io::Result<Vec<Group>> {
    let file = fs::File::open("./data/day-06.txt")?;

    // Each group's answers are separated by a blank line,
    // and within each group, each person's answers are on a single line.

    let mut groups = Vec::new();
    let mut next_group: Option<Group> = None;

    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if t.is_empty() {
            if let Some(grp) = next_group.take() {
                groups.push(grp);
            }
        } else {
            match &mut next_group {
                Some(grp) => grp.push(t.to_string()),
                None => next_group = Some(vec![t.to_string()]),
            }
        }
    }
    if let Some(g) = next_group.take() {
        groups.push(g);
    }

    Ok(groups)
}

pub fn run() {
    println!("--- Day 6: Custom Customs ---");

    let groups = read_input().expect("invalid input");
    // For each group, count the number of questions to which anyone answered "yes".
    // What is the sum of those counts?
    let count: usize = groups
        .iter()
        .map(|grp| {
            let mut answers: HashSet<char> = HashSet::new();
            for user in grp {
                answers.extend(user.chars());
            }
            answers.len()
        })
        .sum();

    println!(
        "Part 1. The sum of questions to which anyone answered \"yes\": {}",
        count
    );

    // --- Part Two ---
    // For each group, count the number of questions to which everyone answered "yes".
    // What is the sum of those counts?
    let count: usize = groups
        .iter()
        .map(|grp| {
            if let Some(first) = grp.first() {
                let mut answers: HashSet<char> = first.chars().collect();

                for next in &grp[1..] {
                    let next_answers: HashSet<char> = next.chars().collect();
                    answers.retain(|ch| next_answers.contains(ch));
                }

                answers.len()
            } else {
                0
            }
        })
        .sum();

    // Your puzzle answer was 3473.
    println!(
        "Part 2. The sum of questions to which everyone answered \"yes\": {}",
        count
    );
}
