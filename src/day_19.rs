use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq)]
enum RuleItem {
    Data(Vec<String>),
    Link(usize),
    Pair(Rule, Rule),
}

type Rule = Vec<RuleItem>;
type Book = HashMap<usize, Rule>;

fn parse_pair<'a>(s: &'a str, splitter: &str) -> Option<(&'a str, &'a str)> {
    s.find(splitter)
        .map(|pos| (s[0..pos].trim(), s[pos + splitter.len()..].trim()))
}

fn parse_rule(s: &str) -> Rule {
    if let Some(pair) = parse_pair(s, "|") {
        vec![RuleItem::Pair(parse_rule(pair.0), parse_rule(pair.1))]
    } else if let Some(pair) = s.find('"').zip(s.rfind('"')) {
        vec![RuleItem::Data(vec![s[pair.0 + 1..pair.1].to_string()])]
    } else {
        s.split_whitespace()
            .map(|s| RuleItem::Link(s.parse().expect("invalid number")))
            .collect()
    }
}

fn read_rules() -> Result<Book, Box<dyn Error>> {
    let file = fs::File::open("./data/day-19-rules.txt")?;

    let mut buf = Book::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if !t.is_empty() {
            let pair = parse_pair(t, ":").expect("invalid rule");
            buf.insert(pair.0.parse()?, parse_rule(pair.1));
        }
    }

    Ok(buf)
}

fn read_messages() -> Result<Vec<String>, Box<dyn Error>> {
    let file = fs::File::open("./data/day-19-messages.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        buf.push(s.trim().to_string());
    }

    Ok(buf)
}

fn has_link(rule: &[RuleItem]) -> bool {
    for i in 0..rule.len() {
        match &rule[i] {
            RuleItem::Link(_) => return true,
            RuleItem::Pair(a, b) => {
                if has_link(a) || has_link(b) {
                    return true;
                }
            }
            RuleItem::Data(_) => {}
        }
    }
    false
}

fn join_data(a: &[String], b: &[String]) -> Vec<String> {
    if a.is_empty() {
        b.iter().cloned().collect()
    } else if b.is_empty() {
        a.iter().cloned().collect()
    } else {
        let mut buf = Vec::new();
        for sa in a {
            for sb in b {
                buf.push(sa.clone() + sb);
            }
        }
        buf
    }
}

// Squeeze out all possible variants
fn extract(rule: &[RuleItem]) -> Vec<String> {
    assert!(!has_link(rule));

    let mut buf = Vec::new();
    for item in rule {
        match item {
            RuleItem::Data(d) => buf = join_data(&buf, d),
            RuleItem::Pair(a, b) => {
                let mut tmp = join_data(&buf, &extract(a));
                tmp.extend(join_data(&buf, &extract(b)).into_iter());
                buf = tmp;
            }
            RuleItem::Link(_) => unreachable!(),
        }
    }
    buf
}

fn is_message_valid(rule: &[RuleItem], mut s: &str) -> bool {
    for r in rule {
        if let RuleItem::Data(d) = r {
            match d.iter().find(|&x| s.starts_with(x)) {
                Some(m) => s = &s[m.len()..],
                None => return false,
            }
        } else {
            unreachable!();
        }
    }
    s.is_empty()
}

fn replace_sub_rule(rule: &mut [RuleItem], rule_id: usize, data: &[String]) {
    for i in 0..rule.len() {
        if let RuleItem::Pair(a, b) = &mut rule[i] {
            replace_sub_rule(a, rule_id, data);
            replace_sub_rule(b, rule_id, data);
        } else if let RuleItem::Link(id) = &rule[i] {
            if *id == rule_id {
                rule[i] = RuleItem::Data(data.iter().cloned().collect());
            }
        }
    }
}

fn get_valid_rule(book: &mut Book) -> Option<(&usize, &Rule)> {
    book.iter().find(|e| !has_link(e.1))
}

fn replace_rule(book: &mut Book, rule_id: usize, data: &[String]) {
    for rule in book.values_mut() {
        replace_sub_rule(rule, rule_id, data);
    }
}

fn match_start<'a>(rule: &[String], mut s: &'a str) -> (usize, &'a str) {
    let mut count = 0;
    while let Some(r) = rule.iter().find(|&r| s.starts_with(r)) {
        s = &s[r.len()..];
        count += 1;
    }
    (count, s)
}

fn match_end<'a>(rule: &[String], mut s: &'a str) -> (usize, &'a str) {
    let mut count = 0;
    while let Some(r) = rule.iter().find(|&r| s.ends_with(r)) {
        s = &s[..s.len() - r.len()];
        count += 1;
    }
    (count, s)
}

pub fn run() {
    println!("--- Day 19: Monster Messages ---");

    let mut book = read_rules().expect("invalid rules");
    let messages = read_messages().expect("invalid messages");

    let mut rule_0 = book.remove(&0).expect("Rule Zero not found");
    let mut rule_42: Option<Vec<String>> = None;
    let mut rule_31: Option<Vec<String>> = None;

    while let Some((&rule_id, rule)) = get_valid_rule(&mut book) {
        let data = extract(rule);

        book.remove(&rule_id);
        replace_rule(&mut book, rule_id, &data);
        replace_sub_rule(&mut rule_0, rule_id, &data);

        // Keep rules 31 and 42 for part 2.
        if rule_id == 42 {
            rule_42 = Some(data);
        } else if rule_id == 31 {
            rule_31 = Some(data);
        }
    }

    // How many messages completely match rule 0?
    let count = messages
        .iter()
        .filter(|&msg| is_message_valid(&rule_0, msg))
        .count();

    // Your puzzle answer was 178.
    println!("Part 1. Number of valid messages: {}", count);

    // --- Part Two ---

    // Replace rules 8: 42 and 11: 42 31 with the following:
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    // Now, the rules do contain loops! But the message length is finite, so:
    // 0: 8 11
    // 8: 42 | 42 42 | 42 42 42 | and so on
    // 11: 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | and so on
    //  =>
    // 0: 42 * (2 + n) 31 * (1 + m)
    // where n >= 0 and n >= m
    let rule_42 = rule_42.expect("Rule 42 not found");
    let rule_31 = rule_31.expect("Rule 31 not found");

    let count = messages
        .iter()
        .filter(|&msg| {
            let (i, s) = match_start(&rule_42, msg);
            let (j, s) = match_end(&rule_31, s);
            s.is_empty() && i >= 2 && j >= 1 && i >= j + 1
        })
        .count();

    // Your puzzle answer was 346.
    println!("Part 2. Number of valid messages: {}", count);
}
