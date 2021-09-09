use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};

// The homework consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)).
#[derive(Debug, Clone)]
enum MathToken {
    Number(i64),
    Plus,
    Multiply,
    OpenParenthesis,
    CloseParenthesis,
    Group(Vec<MathToken>),
}

fn tokenize(s: &str) -> Vec<MathToken> {
    let mut tokens: BTreeMap<usize, MathToken> = BTreeMap::new();
    for (k, v) in s.match_indices(|ch: char| ch.is_digit(10)) {
        tokens.insert(k, MathToken::Number(v.parse().expect("invalid number")));
    }
    for (k, v) in s.chars().enumerate() {
        match v {
            '+' => {
                tokens.insert(k, MathToken::Plus);
            }
            '*' => {
                tokens.insert(k, MathToken::Multiply);
            }
            '(' => {
                tokens.insert(k, MathToken::OpenParenthesis);
            }
            ')' => {
                tokens.insert(k, MathToken::CloseParenthesis);
            }
            _ => (),
        }
    }

    tokens.values().cloned().collect()
}

fn find_close_parenthesis_index(tokens: &[MathToken]) -> Option<usize> {
    let mut level = 1;
    for (i, t) in tokens.iter().enumerate() {
        match t {
            MathToken::OpenParenthesis => {
                level += 1;
            }
            MathToken::CloseParenthesis => {
                level -= 1;
                if level == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn replace_parentheses_with_groups(tokens: &[MathToken]) -> Vec<MathToken> {
    let mut buf = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            MathToken::OpenParenthesis => {
                let j = find_close_parenthesis_index(&tokens[i + 1..])
                    .expect("Could not find closing parenthesis");
                buf.push(MathToken::Group(replace_parentheses_with_groups(
                    &tokens[i + 1..i + 1 + j],
                )));
                i += j + 2;
            }
            t => {
                buf.push(t.clone());
                i += 1;
            }
        }
    }

    buf
}

fn group_additions(tokens: &[MathToken]) -> Vec<MathToken> {
    let mut buf = Vec::new();

    for t in tokens {
        buf.push(match t {
            MathToken::Group(v) => MathToken::Group(group_additions(v)),
            x => x.clone(),
        });
    }

    let mut i = 1;
    while i < buf.len() {
        if let MathToken::Plus = &buf[i] {
            let group = MathToken::Group(buf.drain(i - 1..=i + 1).collect());
            buf.insert(i - 1, group);
        } else {
            i += 2;
        }
    }

    buf
}

fn evaluate(expr: &[MathToken]) -> i64 {
    let mut iter = expr.iter();

    let mut x = match iter.next().expect("Invalid math sentence") {
        MathToken::Number(n) => *n,
        MathToken::Group(t) => evaluate(t),
        _ => panic!("Invalid first token"),
    };

    while let Some(op) = iter.next() {
        let y = match iter.next().expect("Invalid math sentence") {
            MathToken::Number(n) => *n,
            MathToken::Group(t) => evaluate(t),
            _ => panic!("Invalid next token"),
        };

        // The operators have the same precedence, and are evaluated left-to-right
        // regardless of the order in which they appear.
        match op {
            MathToken::Plus => x += y,
            MathToken::Multiply => x *= y,
            _ => panic!("Invalid operation token"),
        }
    }

    x
}

fn read_homework() -> std::io::Result<Vec<Vec<MathToken>>> {
    let file = fs::File::open("./data/day-18.txt")?;

    // Evaluate the expression on each line of the homework.
    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if !t.is_empty() {
            let tokens = tokenize(t);
            let groups = replace_parentheses_with_groups(&tokens);
            buf.push(groups);
        }
    }

    Ok(buf)
}

pub fn run() {
    println!("--- Day 18: Operation Order ---");

    // What is the sum of the resulting values?
    let input = read_homework().expect("Invalid input");
    let sum: i64 = input.iter().map(|expr| evaluate(expr)).sum();

    // Your puzzle answer was 21022630974613.
    println!("Part 1. The sum of the resulting values: {}", sum);

    // --- Part Two ---

    // Now, addition and multiplication have different precedence levels,
    // but they're not the ones you're familiar with.
    // Instead, addition is evaluated before multiplication.

    // What do you get if you add up the results of evaluating
    // the homework problems using these new rules?
    let sum: i64 = input
        .iter()
        .map(|expr| group_additions(expr))
        .map(|expr| evaluate(&expr))
        .sum();

    // Your puzzle answer was 169899524778212.
    println!("Part 2. The sum of the resulting values: {}", sum);
}
