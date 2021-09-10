fn transform(subject_number: usize, loop_size: usize) -> usize {
    // To transform a subject number, start with the value 1.
    let mut value = 1;
    // Then, a number of times called the loop size, perform the following steps:
    for _ in 0..loop_size {
        // Set the value to itself multiplied by the subject number.
        value *= subject_number;
        // Set the value to the remainder after dividing the value by 20201227.
        value %= 20201227;
    }
    value
}

fn get_loop_size(subject_number: usize, result_number: usize) -> usize {
    // To transform a subject number, start with the value 1.
    let mut value = 1;
    // Then, a number of times called the loop size, perform the following steps:
    let mut loop_size = 0;
    loop {
        if value == result_number {
            break;
        }
        // Set the value to itself multiplied by the subject number.
        value *= subject_number;
        // Set the value to the remainder after dividing the value by 20201227.
        value %= 20201227;
        loop_size += 1;
    }

    loop_size
}

pub fn run() {
    println!("--- Day 25: Combo Breaker ---");

    // Your puzzle input:
    let card_pub_key = 15335876;
    let door_pub_key = 15086442;

    let card_loop_size = get_loop_size(7, card_pub_key);
    let door_loop_size = get_loop_size(7, door_pub_key);

    // At this point, you can use either device's loop size with the other device's public key to calculate the encryption key.
    // Transforming the subject number of 17807724 (the door's public key) with a loop size of 8 (the card's loop size) produces the encryption key, 14897079.
    // (Transforming the subject number of 5764801 (the card's public key) with a loop size of 11 (the door's loop size) produces the same encryption key: 14897079.)
    let encryption_key_1 = transform(door_pub_key, card_loop_size);
    let encryption_key_2 = transform(card_pub_key, door_loop_size);
    assert_eq!(encryption_key_1, encryption_key_2);

    // What encryption key is the handshake trying to establish?

    // Your puzzle answer was 11707042.
    println!("The handshake encryption key: {}", encryption_key_1);
}
