use std::fmt;

#[derive(Debug)]
struct CupCircle {
    current_cup: usize,
    nexts: Vec<usize>,
}

impl fmt::Display for CupCircle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut cup = self.current_cup;
        write!(f, "{}", cup)?;
        for _ in 1..self.nexts.len() {
            cup = self.get_next_cup(cup);
            write!(f, " {}", cup)?;
        }
        Ok(())
    }
}

impl CupCircle {
    pub fn cup_index(cup: usize) -> usize {
        cup - 1
    }

    pub fn get_next_cup(&self, cup: usize) -> usize {
        self.nexts[Self::cup_index(cup)]
    }

    fn set_next_cup(&mut self, cup: usize, next_cup: usize) {
        self.nexts[Self::cup_index(cup)] = next_cup;
    }

    pub fn new(mut order: usize, size: usize) -> Self {
        let mut cups = Vec::with_capacity(size);

        while order > 0 {
            cups.push(order % 10);
            order = order / 10;
        }
        cups.reverse();

        while cups.len() < size {
            cups.push(cups.len() + 1);
        }

        let mut nexts = vec![0; size];
        let mut iter = cups.into_iter();
        let current_cup = iter.next().expect("invalid cup size");
        let mut cup = current_cup;

        for next_cup in iter {
            nexts[Self::cup_index(cup)] = next_cup;
            cup = next_cup;
        }
        nexts[Self::cup_index(cup)] = current_cup;

        CupCircle { current_cup, nexts }
    }

    pub fn next_move(&mut self) {
        // The first cup in your list is the current cup.
        let first_cup = self.current_cup;
        // The next three cups are removed from the circle.
        let removed_cup_1 = self.get_next_cup(first_cup);
        let removed_cup_2 = self.get_next_cup(removed_cup_1);
        let removed_cup_3 = self.get_next_cup(removed_cup_2);

        // The crab selects a destination cup: the cup with a label equal to the current cup's
        // label minus one. If this would select one of the cups that was just picked up, the crab
        // will keep subtracting one until it finds a cup that wasn't just picked up. If at any
        // point in this process the value goes below the lowest value on any cup's label, wraps
        // around to the highest value on any cup's label instead.
        let highest = self.nexts.len();
        let mut dest_cup = first_cup;
        loop {
            dest_cup = if dest_cup > 1 { dest_cup - 1 } else { highest };

            if dest_cup != removed_cup_1 && dest_cup != removed_cup_2 && dest_cup != removed_cup_3 {
                break;
            }
        }

        self.current_cup = self.get_next_cup(removed_cup_3);
        self.set_next_cup(first_cup, self.current_cup);
        self.set_next_cup(removed_cup_3, self.get_next_cup(dest_cup));
        self.set_next_cup(dest_cup, removed_cup_1);
    }

    pub fn after_one_order(&self) -> usize {
        let mut order = 0;

        let mut cup = self.get_next_cup(1);
        for _ in 1..self.nexts.len() {
            order = order * 10 + cup;
            cup = self.get_next_cup(cup);
        }
        order
    }

    pub fn two_cups_after_one(&self) -> (usize, usize) {
        let cup_1 = self.get_next_cup(1);
        let cup_2 = self.get_next_cup(cup_1);
        (cup_1, cup_2)
    }
}

pub fn run() {
    println!("--- Day 23: Crab Cups ---");

    // Your puzzle input is 394618527.
    let input = 394618527;

    let mut circle = CupCircle::new(input, 9);
    for _ in 0..100 {
        circle.next_move();
    }

    let order = circle.after_one_order();

    // Your puzzle answer was 78569234.
    println!("Part 1. The labels on the cups after cup 1: {}", order);

    // --- Part Two ---

    // one million cups
    // ten million moves
    let mut circle = CupCircle::new(input, 1_000_000);
    for _ in 0..10_000_000 {
        circle.next_move();
    }

    // The crab is going to hide your stars under the two cups that will end up immediately clockwise of cup 1.
    let cups = circle.two_cups_after_one();

    // Your puzzle answer was 565615814504.
    println!("Part 2. Two cups product: {}", cups.0 * cups.1);
}
