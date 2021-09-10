use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

const TILE_SIZE: usize = 10;

//  -------      -------
// | abcde |    | edcba |
// | fghij |    | jihgf |
// | klmno | => | onmlk |
// | pqrst |    | tsrqp |
// | uvwxy |    | yxwvu |
//  -------      -------
// or as a string: abcdefghijklmnopqrstuvwxy => edcbajihgfonmlktsrqpyxwvu
fn flip(s: &str) -> String {
    let size = (s.len() as f64).sqrt() as usize;
    let len = size * size;
    let mut buf = String::with_capacity(len);

    for i in (0..len).step_by(size) {
        buf.extend(s[i..i + size].chars().rev());
    }

    buf
}

//  -------      -------
// | abcde |    | upkfa |
// | fghij |    | vqlgb |
// | klmno | => | wrmhc |
// | pqrst |    | xsnid |
// | uvwxy |    | ytoje |
//  -------      -------
// or as a string: abcdefghijklmnopqrstuvwxy => upkfavqlgbwrmhcxsnidytoje
fn turn(s: &str) -> String {
    let size = (s.len() as f64).sqrt() as usize;
    let len = size * size;
    let mut buf = String::with_capacity(len);

    for i in (0..size).rev() {
        buf.extend(s.chars().rev().skip(i).step_by(size));
    }

    buf
}

fn all_transforms(orig: String) -> [String; 8] {
    let turn_1 = turn(&orig);
    let turn_2 = turn(&turn_1);
    let turn_3 = turn(&turn_2);

    let flip_1 = flip(&orig);
    let flip_turn_1 = turn(&flip_1);
    let flip_turn_2 = turn(&flip_turn_1);
    let flip_turn_3 = turn(&flip_turn_2);

    [
        orig,
        turn_1,
        turn_2,
        turn_3,
        flip_1,
        flip_turn_1,
        flip_turn_2,
        flip_turn_3,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flipping() {
        let square = "abcdefghijklmnopqrstuvwxy";
        let flip_1 = flip(square);
        let flip_2 = flip(&flip_1);

        assert_eq!(&flip_1, "edcbajihgfonmlktsrqpyxwvu");
        assert_eq!(&flip_2, square);
    }

    #[test]
    fn turning() {
        let square = "abcdefghijklmnopqrstuvwxy";
        let turn_1 = turn(square);
        let turn_2 = turn(&turn_1);
        let turn_3 = turn(&turn_2);
        let turn_4 = turn(&turn_3);

        assert_eq!(&turn_1, "upkfavqlgbwrmhcxsnidytoje");
        assert_eq!(&turn_2, "yxwvutsrqponmlkjihgfedcba");
        assert_eq!(&turn_3, "ejotydinsxchmrwbglqvafkpu");
        assert_eq!(&turn_4, square);
    }

    #[test]
    fn all_transformations() {
        let all = all_transforms("abcd".to_string());

        for trans in [
            "abcd", "cadb", "dcba", "bdac", "badc", "dbca", "cdab", "acbd",
        ] {
            assert!(all.contains(&trans.to_string()));
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn next(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
    pub fn left(&self) -> Self {
        self.next(-1, 0)
    }
    pub fn right(&self) -> Self {
        self.next(1, 0)
    }
    pub fn up(&self) -> Self {
        self.next(0, -1)
    }
    pub fn down(&self) -> Self {
        self.next(0, 1)
    }
}

struct Tile {
    id: usize,
    data: String,
}

fn up_down_equal(a: &str, b: &str) -> bool {
    b.ends_with(&a[..TILE_SIZE])
}

fn left_right_equal(a: &str, b: &str) -> bool {
    a.chars()
        .step_by(TILE_SIZE)
        .zip(b.chars().skip(TILE_SIZE - 1).step_by(TILE_SIZE))
        .all(|(a, b)| a == b)
}

impl Tile {
    pub fn can_add_up(&self, s: &str) -> bool {
        up_down_equal(&self.data, s)
    }
    pub fn can_add_down(&self, s: &str) -> bool {
        up_down_equal(s, &self.data)
    }
    pub fn can_add_left(&self, s: &str) -> bool {
        left_right_equal(&self.data, s)
    }
    pub fn can_add_right(&self, s: &str) -> bool {
        left_right_equal(s, &self.data)
    }
}

struct Image {
    tiles: HashMap<Position, Tile>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pos: Position, id: usize, data: String) {
        assert!(!self.tiles.contains_key(&pos));
        self.tiles.insert(pos, Tile { id, data });
    }

    pub fn can_add(&mut self, data: &str) -> Option<Position> {
        for (pos, t) in &self.tiles {
            if t.can_add_up(&data) {
                return Some(pos.up());
            }
            if t.can_add_down(&data) {
                return Some(pos.down());
            }
            if t.can_add_left(&data) {
                return Some(pos.left());
            }
            if t.can_add_right(&data) {
                return Some(pos.right());
            }
        }
        None
    }

    pub fn top_left(&self) -> Option<(&Position, &Tile)> {
        self.tiles.iter().find(|(pos, _)| {
            !self.tiles.contains_key(&pos.left()) && !self.tiles.contains_key(&pos.up())
        })
    }

    pub fn bottom_right(&self) -> Option<(&Position, &Tile)> {
        self.tiles.iter().find(|(pos, _)| {
            !self.tiles.contains_key(&pos.right()) && !self.tiles.contains_key(&pos.down())
        })
    }

    pub fn get_image(&self) -> Vec<String> {
        let data_size = TILE_SIZE - 2;

        assert!(!self.tiles.is_empty());
        let p0 = self.top_left().expect("no tiles").0;
        let p1 = self.bottom_right().expect("no tiles").0;

        assert!(p1.x >= p0.x);
        assert!(p1.y >= p0.y);
        let w = (1 + p1.x - p0.x) as usize;
        let h = (1 + p1.y - p0.y) as usize;

        let mut data = Vec::with_capacity(h * data_size);
        for row in 0..h {
            for _ in 0..data_size {
                data.push(String::with_capacity(w * data_size));
            }
            for col in 0..w {
                let tile = self
                    .tiles
                    .get(&p0.next(col as i32, row as i32))
                    .expect("invalid image");
                for i in 0..data_size {
                    let offset = (i + 1) * TILE_SIZE + 1;
                    data[row * data_size + i].push_str(&tile.data[offset..offset + data_size]);
                }
            }
        }
        data
    }

    pub fn corners(&self) -> Vec<(&Position, &Tile)> {
        self.tiles
            .iter()
            .filter(|(pos, _)| {
                let mut count = 0;
                if self.tiles.contains_key(&pos.up()) {
                    count += 1;
                }
                if self.tiles.contains_key(&pos.down()) {
                    count += 1;
                }
                if self.tiles.contains_key(&pos.left()) {
                    count += 1;
                }
                if self.tiles.contains_key(&pos.right()) {
                    count += 1;
                }
                count == 2
            })
            .collect()
    }
}

fn read_input() -> Result<Vec<Tile>, Box<dyn Error>> {
    let file = fs::File::open("./data/day-20.txt")?;

    let mut buf = Vec::new();
    let mut next: Option<Tile> = None;
    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if t.is_empty() {
            if let Some(tile) = next.take() {
                assert_eq!(tile.data.len(), TILE_SIZE * TILE_SIZE);
                buf.push(tile);
            }
        } else {
            if let Some(tile) = next.as_mut() {
                tile.data.push_str(t);
            } else {
                // Tile XXXXX:
                assert!(t.starts_with("Tile"));
                assert!(t.ends_with(":"));
                let id: usize = t
                    .find(|ch: char| ch.is_digit(10))
                    .zip(t.rfind(|ch: char| ch.is_digit(10)))
                    .map(|(i, j)| t[i..j + 1].parse().expect("number"))
                    .expect("fail to start a new tile");
                next = Some(Tile {
                    id,
                    data: String::new(),
                });
            }
        }
    }
    if let Some(tile) = next.take() {
        assert_eq!(tile.data.len(), TILE_SIZE * TILE_SIZE);
        buf.push(tile);
    }

    Ok(buf)
}

fn is_one(ch: &char) -> bool {
    *ch == '#'
}

fn is_monster(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(a, b)| !is_one(&a) || a == b)
}

fn count_monsters(sea: &str, width: usize, height: usize, pattern: &[&str]) -> usize {
    let pw = pattern[0].len();
    let ph = pattern.len();

    let mut count = 0;
    for y in 0..height - ph {
        for x in 0..width - pw {
            if pattern
                .iter()
                .enumerate()
                .map(|(i, a)| {
                    let x0 = (y + i) * width + x;
                    (a, &sea[x0..x0 + pw])
                })
                .all(|(a, b)| is_monster(a, b))
            {
                count += 1;
            }
        }
    }
    count
}

pub fn run() {
    println!("--- Day 20: Jurassic Jigsaw ---");

    let mut tiles = read_input().expect("fail to read input");

    let mut image = Image::new();
    if let Some(tile) = tiles.pop() {
        image.insert(Position { x: 0, y: 0 }, tile.id, tile.data);
    }

    let mut trans: Vec<(usize, [String; 8])> = tiles
        .into_iter()
        .map(|t| (t.id, all_transforms(t.data)))
        .collect();

    while trans.len() > 0 {
        for i in (0..trans.len()).rev() {
            for j in 0..trans[i].1.len() {
                if let Some(pos) = image.can_add(&trans[i].1[j]) {
                    let (id, tr) = trans.remove(i);
                    image.insert(pos, id, tr[j].clone());
                    break;
                }
            }
        }
    }

    // What do you get if you multiply together the IDs of the four corner tiles?
    let corners = image.corners();
    assert_eq!(corners.len(), 4);
    let prod: usize = corners.iter().map(|(_, tile)| tile.id).product();

    // Your puzzle answer was 111936085519519.
    println!("Part 1. The product of the 4 corner tiles: {}", prod);

    // --- Part Two ---

    // Now, you're ready to search for sea monsters!
    // When looking for this pattern in the image, the spaces can be anything; only the # need to match.
    let pattern = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let data = image.get_image();
    let size = data.len();

    for sea in all_transforms(data.join("")) {
        let count = count_monsters(&sea, size, size, &pattern);
        if count > 0 {
            // How many '#' are not part of a sea monster?
            let s = sea.chars().filter(is_one).count();
            let m: usize = pattern
                .iter()
                .map(|s| s.chars().filter(is_one).count())
                .sum();
            let roughness = s - m * count;

            // Your puzzle answer was 1792.
            println!("Part 2. The habitat's water roughness: {}", roughness);
            break;
        }
    }
}
