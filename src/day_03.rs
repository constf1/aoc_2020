use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count_trees(map: &Vec<String>, dx: usize, dy: usize) -> usize {
    let mut count = 0;
    let mut x = 0;
    for line in map.iter().step_by(dy) {
        let c = line.chars().nth(x % line.len()).expect("invalid symbol");
        if c == '#' {
            count += 1;
        }
        x += dx;
    }
    count
}

pub fn read_map() -> std::io::Result<Vec<String>> {
    let file = File::open("./data/day-03.txt")?;

    let mut map = Vec::new();
    for line in BufReader::new(file).lines() {
        map.push(line?.trim().to_string());
    }
    Ok(map)
}

pub fn run() {
    println!("--- Day 3: Toboggan Trajectory ---");

    let map = read_map().expect("invalid input");

    // Starting at the top-left corner of your map and following a slope of right 3 and down 1,
    // how many trees would you encounter?
    let count = count_trees(&map, 3, 1);
    // Your puzzle answer was 228.
    println!("Part 1. The trees in the way: {}", count);

    // --- Part Two ---

    // What do you get if you multiply together the number of trees encountered on each of the
    // listed slopes? Slopes:
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 1, down 1.
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let prod = count
        * count_trees(&map, 1, 1)
        * count_trees(&map, 5, 1)
        * count_trees(&map, 7, 1)
        * count_trees(&map, 1, 2);
    // Your puzzle answer was 6818112000.
    println!("Part 2. The product of trees in the way: {}", prod);
}
