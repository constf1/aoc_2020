use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

// Because the tiles are hexagonal, every tile has six neighbors:
// east, southeast, southwest, west, northwest, and northeast.
enum HexDir {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl HexDir {
    // Directions are given in your list, respectively, as e, se, sw, w, nw, and ne.
    fn name(&self) -> &str {
        match self {
            HexDir::East => "e",
            HexDir::SouthEast => "se",
            HexDir::SouthWest => "sw",
            HexDir::West => "w",
            HexDir::NorthWest => "nw",
            HexDir::NorthEast => "ne",
        }
    }

    fn values() -> [HexDir; 6] {
        [
            HexDir::East,
            HexDir::NorthEast,
            HexDir::NorthWest,
            HexDir::West,
            HexDir::SouthWest,
            HexDir::SouthEast,
        ]
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct HexPos {
    x: i32,
    y: i32,
}

impl HexPos {
    pub fn next(&mut self, dir: &HexDir) {
        match dir {
            HexDir::East => self.x += 2,
            HexDir::NorthEast => {
                self.x += 1;
                self.y += 1;
            }
            HexDir::NorthWest => {
                self.x -= 1;
                self.y += 1;
            }
            HexDir::West => self.x -= 2,
            HexDir::SouthWest => {
                self.x -= 1;
                self.y -= 1;
            }
            HexDir::SouthEast => {
                self.x += 1;
                self.y -= 1;
            }
        }
    }

    pub fn neighbor(&self, dir: &HexDir) -> Self {
        let mut pos = self.clone();
        pos.next(dir);
        pos
    }
}

fn read_tile_pos(mut s: &str) -> HexPos {
    let mut pos = HexPos { x: 0, y: 0 };
    while !s.is_empty() {
        let values = HexDir::values();
        let d = values
            .iter()
            .find(|&d| s.starts_with(d.name()))
            .expect("invalid hex direction");
        s = &s[d.name().len()..];
        pos.next(d);
    }
    pos
}

fn read_input() -> std::io::Result<Vec<HexPos>> {
    let file = fs::File::open("./data/day-24.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if !t.is_empty() {
            buf.push(read_tile_pos(t));
        }
    }

    Ok(buf)
}

fn next_exhibit(blacks: &HashSet<HexPos>) -> HashSet<HexPos> {
    let mut next = HashSet::new();
    let mut whites = HashSet::new();

    for pos in blacks {
        let mut count = 0;
        for dir in HexDir::values() {
            let next_pos = pos.neighbor(&dir);
            if blacks.contains(&next_pos) {
                count += 1;
            } else {
                whites.insert(next_pos);
            }
        }
        // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
        if count > 0 && count <= 2 {
            next.insert(pos.clone());
        }
    }

    for pos in whites {
        let mut count = 0;
        for dir in HexDir::values() {
            let next_pos = pos.neighbor(&dir);
            if blacks.contains(&next_pos) {
                count += 1;
            }
        }
        // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
        if count == 2 {
            next.insert(pos.clone());
        }
    }

    next
}

pub fn run() {
    println!("--- Day 24: Lobby Layout ---");

    let input = read_input().expect("invalid input");
    let mut black_tiles: HashSet<HexPos> = HashSet::new();

    for pos in &input {
        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos.clone());
        }
    }

    // Your puzzle answer was 307.
    println!(
        "Part 1. The tiles with the black side up: {}",
        black_tiles.len()
    );

    // --- Part Two ---

    // How many tiles will be black after 100 days?
    for _ in 0..100 {
        black_tiles = next_exhibit(&black_tiles);
    }

    // Your puzzle answer was 3787.
    println!(
        "Part 2. The tiles with the black side up after 100 days: {}",
        black_tiles.len()
    );
}
