fn bus_schedules(s: &str) -> Vec<(usize, usize)> {
    let mut schedules = Vec::new();

    for (delay, bus) in s.split(',').enumerate() {
        if let Ok(bus_id) = bus.parse::<usize>() {
            schedules.push((bus_id, delay));
        }
    }
    schedules
}

pub fn run() {
    println!("--- Day 13: Shuttle Search ---");

    // Your puzzle input:
    let stamp: usize = 1_003_240;
    let outgo = "19,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,787,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,29,x,571,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17";

    // What is the ID of the earliest bus you can take to the airport
    // multiplied by the number of minutes you'll need to wait for that bus?
    let sched = bus_schedules(outgo);
    if let Some((index, time)) = sched
        .iter()
        .map(|(id, _)| (id - stamp % id) % id)
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
    {
        // Your puzzle answer was 3997.
        println!("Part 1. The ID of the earliest bus multiplied by the number of minutes you'll need to wait: {}", sched[index].0 * time);
    }

    // --- Part Two ---

    // What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
    let (mut delta, mut next) = sched[0];
    for i in 1..sched.len() {
        let a = sched[i].0;
        let b = sched[i].1 % a;
        while (next % a) != (a - b) {
            next += delta;
        }
        delta *= a;

        for j in 0..=i {
            assert_eq!((next + sched[j].1) % sched[j].0, 0);
        }
    }

    // Your puzzle answer was 500033211739354.
    println!("Part 2. The earliest timestamp: {}", next);
}
