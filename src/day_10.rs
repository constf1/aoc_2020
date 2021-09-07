use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

fn read_input() -> std::io::Result<Vec<i64>> {
    let file = fs::File::open("./data/day-10.txt")?;

    let mut buf = Vec::new();

    for line in BufReader::new(file).lines() {
        if let Some(n) = line.ok().and_then(|s| s.trim().parse().ok()) {
            buf.push(n);
        }
    }

    Ok(buf)
}

fn get_children(adapters: &[i64], family: &mut HashSet<Vec<i64>>) {
    for i in 2..adapters.len() {
        // Adapters can only connect to a source 1-3 jolts lower than its rating.
        let diff = adapters[i] - adapters[i - 2];
        if diff <= 3 {
            let mut next: Vec<i64> = Vec::new();
            next.extend(adapters[..(i - 1)].iter());
            next.extend(adapters[i..].iter());
            if !family.contains(&next) {
                // Note: all children are shorter than the parent so we can postpone the parent insertion.
                get_children(&next, family);
                assert!(!family.contains(&next));
                family.insert(next);
            }
        }
    }
}

fn count_arrangements(adapters: &[i64]) -> usize {
    let mut family: HashSet<Vec<i64>> = HashSet::new();
    get_children(adapters, &mut family);
    1 + family.len()
}

fn group_adapters(adapters: &[i64]) -> Vec<&[i64]> {
    let mut groups = Vec::new();

    let mut start = 0;
    for i in 1..adapters.len() {
        if adapters[i] >= adapters[i - 1] + 3 {
            groups.push(&adapters[start..i]);
            start = i;
        }
    }
    groups.push(&adapters[start..]);
    groups
}

pub fn run() {
    println!("--- Day 10: Adapter Array ---");

    let mut adapters = read_input().expect("invalid input");
    // The charging outlet has an effective rating of 0 jolts.
    adapters.push(0);
    adapters.sort();

    // Your device's built-in adapter is always 3 higher than the highest adapter.
    adapters.push(adapters.last().expect("empty adpter list") + 3);

    // Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter
    // and count the joltage differences between the charging outlet, the adapters, and your device.
    // What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for i in 1..adapters.len() {
        let diff = adapters[i] - adapters[i - 1];
        // Adapters can only connect to a source 1-3 jolts lower than its rating.
        if diff == 1 {
            diff_1 += 1;
        } else if diff == 3 {
            diff_3 += 1;
        }
    }

    // Your puzzle answer was 2590.
    println!(
        "Part 1. The number of 1-jolt differences multiplied by the number of 3-jolt differences: {}",
        diff_1 * diff_3
    );

    // --- Part Two ---
    // What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?
    let mut count = 1;
    for group in group_adapters(&adapters) {
        count *= count_arrangements(group);
    }

    // Your puzzle answer was 226775649501184.
    println!("Part 2. The number of adapter arangments: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arrangements() {
        let data: &[i64] = &[
            0, 1, 2, 5, 6, 9, 10, 11, 12, 13, 16, 19, 20, 21, 22, 23, 26, 27, 28, 29, 30, 33, 34,
            35, 36, 37, 40, 41, 42, 43, 46, 47, 48, 51, 54, 55, 58, 59, 62, 63, 64, 65, 68, 69, 70,
            73, 76, 77, 78, 81, 82, 83, 84, 87, 90, 91, 92, 93, 94, 97, 98, 99, 100,
        ];
        assert_eq!(count_arrangements(&data[33..34]), 1);
        assert_eq!(count_arrangements(&data[34..36]), 1);
        assert_eq!(count_arrangements(&data[36..38]), 1);

        assert_eq!(count_arrangements(&data[33..38]), 1);

        assert_eq!(count_arrangements(&data[30..33]), 2);
        assert_eq!(count_arrangements(&data[30..38]), 2);

        assert_eq!(count_arrangements(&data[26..30]), 4);
        assert_eq!(count_arrangements(&data[26..38]), 8);

        assert_eq!(count_arrangements(&data[38..42]), 4);
        assert_eq!(count_arrangements(&data[26..42]), 32);

        assert_eq!(count_arrangements(&data[42..45]), 2);
        assert_eq!(count_arrangements(&data[42..46]), 2);
        assert_eq!(count_arrangements(&data[26..46]), 64);
    }
}
