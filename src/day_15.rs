use std::collections::HashMap;

pub fn run() {
    println!("--- Day 15: Rambunctious Recitation ---");

    // Your puzzle input:
    let numbers = [0, 14, 6, 20, 1, 4];

    // Each turn results in that player speaking aloud either 0 (if the last number is new)
    // or an age (if the last number is a repeat)
    let mut number_map: HashMap<usize, usize> = HashMap::new();

    for (val, &key) in numbers.iter().enumerate() {
        number_map.insert(key, val);
    }

    // What will be the 2020th number spoken?
    let mut last = 0;
    for i in numbers.len()..2020 - 1 {
        let next = i - number_map.get(&last).unwrap_or(&i);
        number_map.insert(last, i);
        last = next;
    }

    // Your puzzle answer was 257.
    println!("Part 1. The 2020th number spoken: {}", last);

    // Determine the 30000000th number spoken.
    for i in 2020 - 1..30_000_000 - 1 {
        let next = i - number_map.get(&last).unwrap_or(&i);
        number_map.insert(last, i);
        last = next;
    }

    // Your puzzle answer was 8546398.
    println!("Part 2. The 30000000th number spoken: {}", last);
}
