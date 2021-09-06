use std::fs;
use std::io::{BufRead, BufReader};

// Every seat has a unique seat ID: multiply the row by 8, then add the column.
fn seat_id(row: usize, column: usize) -> usize {
    column + 8 * row
}

fn decode(s: &str) -> Option<(usize, usize)> {
    let mut iter = s.chars();

    // The first 7 characters will either be F or B;
    // these specify exactly one of the 128 rows on the plane (numbered 0 through 127).
    let mut first_row = 0;
    let mut last_row = 127;
    for _ in 0..7 {
        match iter.next() {
            // F means to take the lower half
            Some('F') => last_row = (last_row + first_row - 1) / 2,
            // B means to take the upper half
            Some('B') => first_row = (last_row + first_row + 1) / 2,
            _ => return None,
        }
    }

    // The last three characters will be either L or R;
    // these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7).
    let mut first_column = 0;
    let mut last_column = 7;
    for _ in 0..3 {
        match iter.next() {
            // L means to keep the lower half
            Some('L') => last_column = (last_column + first_column - 1) / 2,
            // R means to keep the upper half
            Some('R') => first_column = (last_column + first_column + 1) / 2,
            _ => return None,
        }
    }

    assert_eq!(first_row, last_row);
    assert_eq!(first_column, last_column);

    Some((first_row, first_column))
}

fn read_input() -> std::io::Result<Vec<String>> {
    let file = fs::File::open("./data/day-05.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        buf.push(line?.trim().to_string());
    }

    Ok(buf)
}

pub fn run() {
    println!("--- Day 5: Binary Boarding ---");
    let items = read_input().expect("invalid input");

    // What is the highest seat ID on a boarding pass?
    let mut ids: Vec<usize> = items
        .iter()
        .map(|s| {
            let (row, col) = decode(s).expect("invalid decoding");
            seat_id(row, col)
        })
        .collect();
    ids.sort();

    if let Some(id) = ids.last() {
        // Your puzzle answer was 955.
        println!("Part 1. The highest seat ID on a boarding pass: {}", id);
    }

    // --- Part Two ---
    // the seats with IDs +1 and -1 from yours will be in your list.
    for i in 1..ids.len() {
        if ids[i - 1] + 2 == ids[i] {
            // Your puzzle answer was 569.
            println!("Part 2. The ID of your seat: {}", ids[i - 1] + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decoding() {
        let (row, col) = decode("FBFBBFFRLR").expect("invalid decoding");
        // decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
        assert_eq!(row, 44);
        assert_eq!(col, 5);
        assert_eq!(seat_id(row, col), 357);
        // Here are some other boarding passes:
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        let (row, col) = decode("BFFFBBFRRR").expect("invalid decoding");
        assert_eq!(row, 70);
        assert_eq!(col, 7);
        assert_eq!(seat_id(row, col), 567);
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        let (row, col) = decode("FFFBBBFRRR").expect("invalid decoding");
        assert_eq!(row, 14);
        assert_eq!(col, 7);
        assert_eq!(seat_id(row, col), 119);
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let (row, col) = decode("BBFFBBFRLL").expect("invalid decoding");
        assert_eq!(row, 102);
        assert_eq!(col, 4);
        assert_eq!(seat_id(row, col), 820);
    }
}
