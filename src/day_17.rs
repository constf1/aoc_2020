use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct ConwayCube(i64, i64, i64, i64);

impl ConwayCube {
    pub fn neighbours_3d(&self) -> ConwayCubeNeighbours {
        ConwayCubeNeighbours {
            cube: *self,
            // Each cube only ever considers its neighbors:
            // any of the 26 other cubes where any of their coordinates differ by at most 1.
            index: 26,
        }
    }
    pub fn neighbours_4d(&self) -> ConwayCubeNeighbours {
        ConwayCubeNeighbours {
            cube: *self,
            // Each cube only ever considers its neighbors:
            // any of the 80 other cubes where any of their coordinates differ by at most 1.
            index: 80,
        }
    }
}

struct ConwayCubeNeighbours {
    cube: ConwayCube,
    index: i64,
}

fn shift(n: i64) -> i64 {
    match n % 3 {
        2 => -1,
        x => x,
    }
}

impl Iterator for ConwayCubeNeighbours {
    type Item = ConwayCube;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            let i = self.index;
            self.index -= 1;
            return Some(ConwayCube(
                self.cube.0 + shift(i),
                self.cube.1 + shift(i / 3),
                self.cube.2 + shift(i / 3 / 3),
                self.cube.3 + shift(i / 3 / 3 / 3),
            ));
        }
        None
    }
}

struct PowerGrid {
    grid: HashSet<ConwayCube>,
}

impl PowerGrid {
    pub fn from_str(s: &str) -> Self {
        let mut grid = HashSet::new();
        let mut y = 0;
        for line in s.split('\n') {
            let t = line.trim();
            if !t.is_empty() {
                for (x, _) in t.match_indices('#') {
                    grid.insert(ConwayCube(x as i64, y, 0, 0));
                }
                y += 1;
            }
        }

        Self { grid }
    }

    pub fn next_3d(&self) -> Self {
        let mut next = HashSet::new();
        let mut inactive = HashSet::new();
        for cube in &self.grid {
            let mut cube_count = 0;
            for neighbour in cube.neighbours_3d() {
                if self.grid.contains(&neighbour) {
                    cube_count += 1;
                } else {
                    inactive.insert(neighbour);
                }
            }
            // If a cube is active and exactly 2 or 3 of its neighbors are also active,
            // the cube remains active. Otherwise, the cube becomes inactive.
            if cube_count == 2 || cube_count == 3 {
                next.insert(*cube);
            }
        }

        for cube in &inactive {
            let mut cube_count = 0;
            for neighbour in cube.neighbours_3d() {
                if self.grid.contains(&neighbour) {
                    cube_count += 1;
                }
            }
            // If a cube is inactive but exactly 3 of its neighbors are active,
            // the cube becomes active. Otherwise, the cube remains inactive.
            if cube_count == 3 {
                next.insert(*cube);
            }
        }

        Self { grid: next }
    }

    pub fn next_4d(&self) -> Self {
        let mut next = HashSet::new();
        let mut inactive = HashSet::new();
        for cube in &self.grid {
            let mut cube_count = 0;
            for neighbour in cube.neighbours_4d() {
                if self.grid.contains(&neighbour) {
                    cube_count += 1;
                } else {
                    inactive.insert(neighbour);
                }
            }

            // If a cube is active and exactly 2 or 3 of its neighbors are also active,
            // the cube remains active. Otherwise, the cube becomes inactive.
            if cube_count == 2 || cube_count == 3 {
                next.insert(*cube);
            }
        }

        for cube in &inactive {
            let mut cube_count = 0;
            for neighbour in cube.neighbours_4d() {
                if self.grid.contains(&neighbour) {
                    cube_count += 1;
                }
            }
            // If a cube is inactive but exactly 3 of its neighbors are active,
            // the cube becomes active. Otherwise, the cube remains inactive.
            if cube_count == 3 {
                next.insert(*cube);
            }
        }

        Self { grid: next }
    }

    pub fn grid(&self) -> &HashSet<ConwayCube> {
        &self.grid
    }
}

const GRID: &str = r"#...#.#.
..#.#.##
..#..#..
.....###
...#.#.#
#.#.##..
#####...
.#.#.##.";

pub fn run() {
    println!("--- Day 17: Conway Cubes ---");

    let grid = PowerGrid::from_str(GRID);

    // How many 3d cubes are left in the active state after the sixth cycle?
    let next = grid
        .next_3d()
        .next_3d()
        .next_3d()
        .next_3d()
        .next_3d()
        .next_3d();

    // Your puzzle answer was 401.
    println!("Part 1. Active 3d cubes: {}", next.grid().len());

    // --- Part Two ---

    // How many 4d cubes are left in the active state after the sixth cycle?
    let next = grid
        .next_4d()
        .next_4d()
        .next_4d()
        .next_4d()
        .next_4d()
        .next_4d();

    // Your puzzle answer was 2224.
    println!("Part 2. Active 4d cubes: {}", next.grid().len());
}
