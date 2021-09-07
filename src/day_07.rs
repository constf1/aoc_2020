use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

type Bag = HashMap<String, usize>;
type BagMap = HashMap<String, Bag>;

fn count_bags(bag_map: &BagMap, bag_color: &str) -> usize {
    let mut total = 1;
    for (bag, count) in bag_map.get(bag_color).expect("invalid bag map").iter() {
        total = total + count_bags(bag_map, bag) * count;
    }
    total
}

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

fn read_number<'a>(s: &'a str) -> Option<(usize, &'a str)> {
    s.find(|ch: char| !ch.is_digit(10))
        .and_then(|pos| match s[0..pos].parse() {
            Ok(n) => Some((n, s[pos..].trim())),
            _ => None,
        })
}

// content: <number> <color-name> bag[s], <number> <color-name> bag[s].
//      or: no other bags.
fn new_bag(mut content: &str) -> Bag {
    let splitter = "bag";
    let mut bag = Bag::new();
    while let Some((n, rest)) = read_number(content) {
        let (color, mut data) = parse_pair(rest, splitter).expect("invalid bag name");
        bag.insert(color.to_string(), n);
        // Consume 's' or ',' or '.'
        while data.starts_with("s") || data.starts_with(",") || data.starts_with(".") {
            data = &data[1..];
        }
        content = data.trim();
    }
    bag
}

fn read_input() -> std::io::Result<BagMap> {
    let file = fs::File::open("./data/day-07.txt")?;

    let mut bags = BagMap::new();

    // Each key: value pairs are separated by 'bags contain'.
    let splitter = "bags contain";
    for line in BufReader::new(file).lines() {
        let s = line?;
        if let Some((key, val)) = parse_pair(&s, splitter) {
            bags.insert(key.to_string(), new_bag(val));
        }
    }

    Ok(bags)
}

pub fn run() {
    println!("--- Day 7: Handy Haversacks ---");

    let bag_map = read_input().expect("invalid input");

    // How many bag colors can eventually contain at least one "shiny gold" bag?
    let our_color = "shiny gold";
    let mut valid_bags: HashSet<String> = HashSet::new();
    for (color, bags) in &bag_map {
        if bags.contains_key(our_color) {
            valid_bags.insert(color.clone());
        }
    }
    let mut count = valid_bags.len();
    loop {
        for (color, bags) in &bag_map {
            for valid_color in &valid_bags {
                if bags.contains_key(valid_color) {
                    valid_bags.insert(color.clone());
                    break;
                }
            }
        }
        if count == valid_bags.len() {
            break;
        }
        count = valid_bags.len();
    }

    // Your puzzle answer was 155.
    println!("Part 1. Number of bag colors: {}", count);

    // --- Part Two ---
    // How many individual bags are required inside your single shiny gold bag?
    let count = count_bags(&bag_map, our_color);

    // Your puzzle answer was 54803.
    println!("Part 2. Our bas has {} other bags.", count - 1);
}
