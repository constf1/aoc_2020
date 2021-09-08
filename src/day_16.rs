use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Rule {
    name: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

fn parse_range_inclusive<Idx>(s: &str, splitter: &str) -> Option<RangeInclusive<Idx>>
where
    Idx: FromStr,
{
    let pair = parse_pair(s, splitter)?;
    let start = pair.0.parse().ok()?;
    let end = pair.1.parse().ok()?;

    Some(start..=end)
}

fn read_rules() -> Result<Vec<Rule>, Box<dyn Error>> {
    let file = File::open("./data/day-16-rules.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let pair = parse_pair(&s, ":").expect("invalid field");
        let ranges = parse_pair(pair.1, "or").expect("invalid range pair");

        buf.push(Rule {
            name: pair.0.to_string(),
            range_1: parse_range_inclusive(ranges.0, "-").expect("invalid range 1"),
            range_2: parse_range_inclusive(ranges.1, "-").expect("invalid range 2"),
        });
    }

    Ok(buf)
}

fn read_tickets() -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let file = File::open("./data/day-16-tickets.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let s = s.trim();
        if !s.is_empty() {
            let mut ticket = Vec::new();
            for field in s.split(',') {
                ticket.push(field.trim().parse()?);
            }
            buf.push(ticket);
        }
    }

    Ok(buf)
}

fn is_valid_any_rule(rules: &Vec<Rule>, val: &usize) -> bool {
    rules
        .iter()
        .any(|r| r.range_1.contains(val) || r.range_2.contains(val))
}

fn is_ticket_valid(rules: &Vec<Rule>, ticket: &Vec<usize>) -> bool {
    ticket.iter().all(|val| is_valid_any_rule(rules, val))
}

pub fn run() {
    println!("--- Day 16: Ticket Translation ---");

    let rules = read_rules().expect("invalid rules");
    let mut tickets = read_tickets().expect("invalid tickets");

    // Start by determining which tickets are completely invalid;
    // these are tickets that contain values which aren't valid for any field.
    // Ignore your ticket for now.
    // Adding together all of the invalid values produces your ticket scanning error rate.
    let mut rate = 0;
    for i in 1..tickets.len() {
        for val in &tickets[i] {
            if !is_valid_any_rule(&rules, val) {
                rate += val;
            }
        }
    }

    // Your puzzle answer was 23044.
    println!("Part 1. Ticket scanning error rate: {}", rate);

    // --- Part Two ---

    // Now that you've identified which tickets contain invalid values, discard those tickets entirely.
    tickets.retain(|t| is_ticket_valid(&rules, t));

    // Using the valid ranges for each field, determine what order the fields appear on the tickets.
    let mut variants: Vec<Vec<usize>> = Vec::new();
    for r_index in 0..rules.len() {
        let mut feasible: Vec<usize> = Vec::new();
        for t_index in 0..rules.len() {
            if tickets
                .iter()
                .map(|t| &t[t_index])
                .all(|v| rules[r_index].range_1.contains(v) || rules[r_index].range_2.contains(v))
            {
                feasible.push(t_index);
            }
        }
        variants.push(feasible);
    }

    // Work out which field is which.
    let mut remapper: HashMap<usize, usize> = HashMap::new();
    loop {
        let len = remapper.len();
        for (k, v) in variants.iter().enumerate() {
            if v.len() == 1 {
                remapper.insert(v[0], k);
            }
        }
        assert_ne!(remapper.len(), len);
        if remapper.len() == variants.len() {
            break;
        }

        for v in variants.iter_mut() {
            v.retain(|k| !remapper.contains_key(k));
        }
    }

    // Look for the 6 fields on your ticket that start with the word departure.
    // What do you get if you multiply those 6 values together?
    let product: usize = remapper
        .iter()
        .filter(|(_, &v)| rules[v].name.starts_with("departure"))
        .map(|(&k, _)| tickets[0][k])
        .product();

    // Your puzzle answer was 3765150732757.
    println!("Part 2. Departure product: {}", product);
}
