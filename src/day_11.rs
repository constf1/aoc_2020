use std::fs;
use std::io::{BufRead, BufReader};

// The seat layout fits neatly on a grid.
// Each position is either floor (.), an empty seat (L), or an occupied seat (#).
const GRID_FLOOR: u8 = 0;
const GRID_EMPTY_SEAT: u8 = 1;
const GRID_OCCUPIED_SEAT: u8 = 2;

fn read_input() -> std::io::Result<(Vec<u8>, usize)> {
    let file = fs::File::open("./data/day-11.txt")?;

    let mut buf = Vec::new();
    let mut rows = 0;
    for line in BufReader::new(file).lines() {
        if let Ok(n) = line {
            let row = n.trim();
            if !row.is_empty() {
                rows += 1;
                for ch in row.chars() {
                    if ch == '.' {
                        buf.push(GRID_FLOOR);
                    } else if ch == 'L' {
                        buf.push(GRID_EMPTY_SEAT);
                    } else if ch == '#' {
                        buf.push(GRID_OCCUPIED_SEAT);
                    } else {
                        panic!("Unknown character in the grid: {}", ch);
                    }
                }
            }
        }
    }

    Ok((buf, rows))
}

fn can_occupy_v1(grid: &[u8], rows: usize, index: usize) -> bool {
    // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    let cols = grid.len() / rows;
    let x = index % cols;
    let y = index / cols;
    // left
    if x > 0 {
        if grid[index - 1] == GRID_OCCUPIED_SEAT {
            return false;
        }
        // left-up
        if y > 0 {
            if grid[index - 1 - cols] == GRID_OCCUPIED_SEAT {
                return false;
            }
        }
        // left-down
        if y + 1 < rows {
            if grid[index - 1 + cols] == GRID_OCCUPIED_SEAT {
                return false;
            }
        }
    }
    // right
    if x + 1 < cols {
        if grid[index + 1] == GRID_OCCUPIED_SEAT {
            return false;
        }
        // right-up
        if y > 0 {
            if grid[index + 1 - cols] == GRID_OCCUPIED_SEAT {
                return false;
            }
        }
        // right-down
        if y + 1 < rows {
            if grid[index + 1 + cols] == GRID_OCCUPIED_SEAT {
                return false;
            }
        }
    }
    // up
    if y > 0 {
        if grid[index - cols] == GRID_OCCUPIED_SEAT {
            return false;
        }
    }
    // down
    if y + 1 < rows {
        if grid[index + cols] == GRID_OCCUPIED_SEAT {
            return false;
        }
    }
    true
}

fn can_see_occupied<T>(grid: &[u8], iter: T) -> bool
where
    T: Iterator<Item = usize>,
{
    for index in iter {
        let seat = grid[index];
        if seat == GRID_OCCUPIED_SEAT {
            return true;
        } else if seat == GRID_EMPTY_SEAT {
            break;
        }
    }
    return false;
}

fn can_occupy_v2(grid: &[u8], rows: usize, index: usize) -> bool {
    // If a seat is empty (L) and there are no occupied seats in each of 8 direction, the seat becomes occupied.
    let cols = grid.len() / rows;
    let x = index % cols;
    let y = index / cols;

    let l_count = x;
    let r_count = cols - x - 1;
    let u_count = y;
    let d_count = rows - y - 1;

    let lu_count = l_count.min(u_count);
    let ld_count = l_count.min(d_count);
    let ru_count = r_count.min(u_count);
    let rd_count = r_count.min(d_count);

    // left-up or right-down diagonal (Major diagonal).
    let lurd = cols + 1;
    // left-down or right-up diagonal (Minor diagonal).
    let ldru = cols - 1;

    // left
    !(can_see_occupied(&grid, (0..l_count).map(|i| index - 1 - i))
    // right
     || can_see_occupied(&grid, (0..r_count).map(|i| index + 1 + i))
    // up
     || can_see_occupied(&grid, (0..u_count).map(|i| index - cols - cols * i))
    // down
     || can_see_occupied(&grid, (0..d_count).map(|i| index + cols * i + cols))
    // left-up
     || can_see_occupied(&grid, (0..lu_count).map(|i| index - lurd - lurd * i))
    // left-down
     || can_see_occupied(&grid, (0..ld_count).map(|i| index + ldru + ldru * i))
    // right-up
     || can_see_occupied(&grid, (0..ru_count).map(|i| index - ldru - ldru * i))
    // right-down
     || can_see_occupied(&grid, (0..rd_count).map(|i| index + lurd + lurd * i)))
}

fn should_empty_v1(grid: &[u8], rows: usize, index: usize) -> bool {
    // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    let mut count = 0;
    let cols = grid.len() / rows;
    let x = index % cols;
    let y = index / cols;
    // left
    if x > 0 {
        if grid[index - 1] == GRID_OCCUPIED_SEAT {
            count += 1;
        }
        // left-up
        if y > 0 {
            if grid[index - 1 - cols] == GRID_OCCUPIED_SEAT {
                count += 1;
            }
        }
        // left-down
        if y + 1 < rows {
            if grid[index - 1 + cols] == GRID_OCCUPIED_SEAT {
                count += 1;
            }
        }
    }
    // right
    if x + 1 < cols {
        if grid[index + 1] == GRID_OCCUPIED_SEAT {
            count += 1;
        }
        // right-up
        if y > 0 {
            if grid[index + 1 - cols] == GRID_OCCUPIED_SEAT {
                count += 1;
            }
        }
        // right-down
        if y + 1 < rows {
            if grid[index + 1 + cols] == GRID_OCCUPIED_SEAT {
                count += 1;
            }
        }
    }
    // up
    if y > 0 {
        if grid[index - cols] == GRID_OCCUPIED_SEAT {
            count += 1;
        }
    }
    // down
    if y + 1 < rows {
        if grid[index + cols] == GRID_OCCUPIED_SEAT {
            count += 1;
        }
    }
    count >= 4
}

fn should_empty_v2(grid: &[u8], rows: usize, index: usize) -> bool {
    // It now takes 5 or more visible occupied seats for an occupied seat to become empty.
    let cols = grid.len() / rows;
    let x = index % cols;
    let y = index / cols;

    let l_count = x;
    let r_count = cols - x - 1;
    let u_count = y;
    let d_count = rows - y - 1;

    let lu_count = l_count.min(u_count);
    let ld_count = l_count.min(d_count);
    let ru_count = r_count.min(u_count);
    let rd_count = r_count.min(d_count);

    // left-up or right-down diagonal (Major diagonal).
    let lurd = cols + 1;
    // left-down or right-up diagonal (Minor diagonal).
    let ldru = cols - 1;

    [
        // left
        can_see_occupied(&grid, (0..l_count).map(|i| index - 1 - i)),
        // right
        can_see_occupied(&grid, (0..r_count).map(|i| index + 1 + i)),
        // up
        can_see_occupied(&grid, (0..u_count).map(|i| index - cols - cols * i)),
        // down
        can_see_occupied(&grid, (0..d_count).map(|i| index + cols * i + cols)),
        // left-up
        can_see_occupied(&grid, (0..lu_count).map(|i| index - lurd - lurd * i)),
        // left-down
        can_see_occupied(&grid, (0..ld_count).map(|i| index + ldru + ldru * i)),
        // right-up
        can_see_occupied(&grid, (0..ru_count).map(|i| index - ldru - ldru * i)),
        // right-down
        can_see_occupied(&grid, (0..rd_count).map(|i| index + lurd + lurd * i)),
    ]
    .iter()
    .filter(|&v| *v)
    .count()
        >= 5
}

fn next_seat<F1, F2>(grid: &[u8], rows: usize, index: usize, can_occupy: F1, should_empty: F2) -> u8
where
    F1: Fn(&[u8], usize, usize) -> bool,
    F2: Fn(&[u8], usize, usize) -> bool,
{
    let seat = grid[index];

    if seat == GRID_EMPTY_SEAT {
        if can_occupy(&grid, rows, index) {
            return GRID_OCCUPIED_SEAT;
        }
    } else if seat == GRID_OCCUPIED_SEAT {
        if should_empty(&grid, rows, index) {
            return GRID_EMPTY_SEAT;
        }
    }
    seat
}

fn next_seat_v1(grid: &[u8], rows: usize, index: usize) -> u8 {
    next_seat(grid, rows, index, can_occupy_v1, should_empty_v1)
}

fn next_seat_v2(grid: &[u8], rows: usize, index: usize) -> u8 {
    next_seat(grid, rows, index, can_occupy_v2, should_empty_v2)
}

fn next_grid<F>(grid: &[u8], rows: usize, next_seat: F) -> Vec<u8>
where
    F: Fn(&[u8], usize, usize) -> u8,
{
    let mut buf = Vec::with_capacity(grid.len());

    for index in 0..grid.len() {
        buf.push(next_seat(grid, rows, index));
    }

    buf
}

fn next_grid_v1(grid: &[u8], rows: usize) -> Vec<u8> {
    next_grid(grid, rows, next_seat_v1)
}

fn next_grid_v2(grid: &[u8], rows: usize) -> Vec<u8> {
    next_grid(grid, rows, next_seat_v2)
}

// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
fn settle_down<F>(grid: &[u8], rows: usize, settler: F) -> Vec<u8>
where
    F: Fn(&[u8], usize) -> Vec<u8>,
{
    let next = settler(grid, rows);
    if next == *grid {
        next
    } else {
        settle_down::<F>(&next, rows, settler)
    }
}

pub fn run() {
    println!("--- Day 11: Seating System ---");

    let (grid, rows) = read_input().expect("invalid input");

    // How many seats end up occupied?
    let count = settle_down(&grid, rows, next_grid_v1)
        .iter()
        .filter(|&s| *s == GRID_OCCUPIED_SEAT)
        .count();

    // Your puzzle answer was 2249.
    println!("Part 1. Occupied seats: {}", count);

    // --- Part Two ---

    let count = settle_down(&grid, rows, next_grid_v2)
        .iter()
        .filter(|&s| *s == GRID_OCCUPIED_SEAT)
        .count();

    // Your puzzle answer was 2023.
    println!("Part 2. Occupied seats: {}", count);
}
