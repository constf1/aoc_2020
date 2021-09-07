use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    pub fn from_rotation(degrees: i64) -> Self {
        match (degrees % 360 + 360) % 360 {
            90 => Action::South,
            180 => Action::West,
            270 => Action::North,
            _ => Action::East,
        }
    }
}

struct Command {
    action: Action,
    amount: i64,
}

#[derive(Debug, Clone)]
pub struct CommandParseError {
    pub message: String,
}

impl CommandParseError {
    pub fn invalid_action(action: Option<char>) -> Self {
        let message = match action {
            Some(ch) => format!("Invalid action: {}", ch),
            None => String::from("No action was found!"),
        };
        Self { message }
    }

    pub fn invalid_amount() -> Self {
        Self {
            message: String::from("Invalid amount!"),
        }
    }
}

// Errors should implement the std::error::Error trait,
// but the default definitions for the Error methods are fine.
impl std::error::Error for CommandParseError {}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.chars().next() {
            Some('N') => Action::North,
            Some('S') => Action::South,
            Some('E') => Action::East,
            Some('W') => Action::West,
            Some('L') => Action::Left,
            Some('R') => Action::Right,
            Some('F') => Action::Forward,
            ch => return Err(CommandParseError::invalid_action(ch)),
        };
        if let Ok(amount) = s[1..].parse() {
            Ok(Self { action, amount })
        } else {
            Err(CommandParseError::invalid_amount())
        }
    }
}

fn read_input() -> Result<Vec<Command>, Box<dyn Error>> {
    let file = fs::File::open("./data/day-12.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let row = s.trim();
        if !row.is_empty() {
            buf.push(row.parse()?);
        }
    }

    Ok(buf)
}

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    // Rotates a point 90 degrees clockwise about the origin. (x,y) -> (y, -x)
    pub fn turn_right(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
    // Rotates a point 180 degrees clockwise about the origin. (x,y) -> (-x, -y)
    pub fn turn_back(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    // Rotates a point 90 degrees counterclockwise about the origin. (x,y) -> (-y, x)
    pub fn turn_left(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn mul(&self, amount: i64) -> Self {
        Self {
            x: self.x * amount,
            y: self.y * amount,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    // Sum of the absolute values of its east/west position and its north/south position
    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

struct Ship {
    pos: Point,
    rot: i64,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            pos: Point { x: 0, y: 0 },
            rot: 0,
        }
    }
    pub fn next_move(&mut self, action: Action, amount: i64) {
        match action {
            // Action N means to move north by the given value.
            Action::North => self.pos.y += amount,
            // Action S means to move south by the given value.
            Action::South => self.pos.y -= amount,
            // Action E means to move east by the given value.
            Action::East => self.pos.x += amount,
            // Action W means to move west by the given value.
            Action::West => self.pos.x -= amount,
            // Action L means to turn left the given number of degrees.
            Action::Left => self.rot -= amount,
            // Action R means to turn right the given number of degrees.
            Action::Right => self.rot += amount,
            // Action F means to move forward by the given value in the direction the ship is currently facing.
            Action::Forward => self.next_move(Action::from_rotation(self.rot), amount),
        }
    }
}

struct ShipWithWaypoint {
    ship: Ship,
    waypoint: Point,
}

impl ShipWithWaypoint {
    pub fn new() -> Self {
        ShipWithWaypoint {
            ship: Ship::new(),
            // The waypoint starts 10 units east and 1 unit north relative to the ship.
            waypoint: Point { x: 10, y: 1 },
        }
    }

    pub fn next_move(&mut self, action: Action, amount: i64) {
        match action {
            // Action N means to move the waypoint north by the given value.
            Action::North => self.waypoint.y += amount,
            // Action S means to move the waypoint south by the given value.
            Action::South => self.waypoint.y -= amount,
            // Action E means to move the waypoint east by the given value.
            Action::East => self.waypoint.x += amount,
            // Action W means to move the waypoint west by the given value.
            Action::West => self.waypoint.x -= amount,
            // Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
            Action::Left => {
                if amount == 90 {
                    self.waypoint = self.waypoint.turn_left();
                } else if amount == 180 {
                    self.waypoint = self.waypoint.turn_back();
                } else if amount == 270 {
                    self.waypoint = self.waypoint.turn_right();
                }
            }
            // Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
            Action::Right => {
                if amount == 90 {
                    self.waypoint = self.waypoint.turn_right();
                } else if amount == 180 {
                    self.waypoint = self.waypoint.turn_back();
                } else if amount == 270 {
                    self.waypoint = self.waypoint.turn_left();
                }
            }
            // Action F means to move forward to the waypoint a number of times equal to the given value.
            Action::Forward => self.ship.pos = self.ship.pos.add(&self.waypoint.mul(amount)),
        }
    }
}

pub fn run() {
    println!("--- Day 12: Rain Risk ---");

    let input = read_input().expect("invalid input");

    // Figure out where the navigation instructions lead.
    // What is the Manhattan distance between that location and the ship's starting position?
    let mut ship_v1 = Ship::new();
    let mut ship_v2 = ShipWithWaypoint::new();

    for cmd in &input {
        ship_v1.next_move(cmd.action, cmd.amount);
        ship_v2.next_move(cmd.action, cmd.amount);
    }

    // Your puzzle answer was 998.
    println!(
        "Part 1. The ship's Manhattan distance: {}",
        ship_v1.pos.manhattan_distance()
    );

    // --- Part Two ---

    // Figure out where the navigation instructions actually lead.
    // What is the Manhattan distance between that location and the ship's starting position?

    // Your puzzle answer was 71586.
    println!(
        "Part 2. The ship's real Manhattan distance: {}",
        ship_v2.ship.pos.manhattan_distance()
    );
}
