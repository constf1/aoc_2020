use std::collections::HashSet;

fn game_score(cards: &[usize]) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i + 1) * v)
        .sum()
}

fn combat(mut player_1: Vec<usize>, mut player_2: Vec<usize>) -> (bool, Vec<usize>) {
    // The first round begins with both players drawing the top card of their decks: 4 and 12.
    // Player 2 has the higher card, so both cards move to the bottom of player 2's deck such that 12 is above 4.
    // When a player has all of the cards, the game ends.
    while !(player_1.is_empty() || player_2.is_empty()) {
        let a = player_1.remove(0);
        let b = player_2.remove(0);
        if a > b {
            player_1.push(a);
            player_1.push(b);
        } else if b > a {
            player_2.push(b);
            player_2.push(a);
        } else {
            unreachable!();
        }
    }

    // Once the game ends, you can calculate the winning player's score.
    return if player_1.is_empty() {
        (false, player_2)
    } else {
        (true, player_1)
    };
}

fn recursive_combat(mut player_1: Vec<usize>, mut player_2: Vec<usize>) -> (bool, Vec<usize>) {
    let mut done = HashSet::new();

    while !(player_1.is_empty() || player_2.is_empty()) {
        // Before either player deals a card, if there was a previous round in this game that had
        // exactly the same cards in the same order in the same players' decks, the game instantly ends
        // in a win for player 1. Previous rounds from other games are not considered. (This prevents
        // infinite games of Recursive Combat, which everyone agrees is a bad idea.)
        let cards = format!("{:?};{:?}", player_1, player_2);
        if done.contains(&cards) {
            return (true, player_1);
        } else {
            done.insert(cards);
        }

        // The players begin the round by each drawing the top card of their deck.
        let a = player_1.remove(0);
        let b = player_2.remove(0);

        // If both players have at least as many cards remaining in their deck
        // as the value of the card they just drew, the winner of the round is
        // determined by playing a new game of Recursive Combat.
        if a <= player_1.len() && b <= player_2.len() {
            // The winner of the round is determined by recursing into a sub-game of Recursive Combat.
            // To play a sub-game of Recursive Combat, each player creates a new deck by making a copy
            // of the next cards in their deck (the quantity of cards copied is equal to the number on
            // the card they drew to trigger the sub-game).
            // During this sub-game, the game that triggered it is on hold and completely unaffected;
            // no cards are removed from players' decks to form the sub-game.
            let (is_first, _) = recursive_combat(
                player_1.iter().take(a).copied().collect(),
                player_2.iter().take(b).copied().collect(),
            );
            if is_first {
                player_1.push(a);
                player_1.push(b);
            } else {
                player_2.push(b);
                player_2.push(a);
            }
        } else {
            // The winner of the round is the player with the higher-value card.
            if a > b {
                player_1.push(a);
                player_1.push(b);
            } else if b > a {
                player_2.push(b);
                player_2.push(a);
            } else {
                unreachable!();
            }
        }
    }

    return if player_1.is_empty() {
        (false, player_2)
    } else {
        (true, player_1)
    };
}

pub fn run() {
    println!("--- Day 22: Crab Combat ---");

    let player_1 = vec![
        50, 19, 40, 22, 7, 4, 3, 16, 34, 45, 46, 39, 44, 32, 20, 29, 15, 35, 41, 2, 21, 28, 6, 26,
        48,
    ];

    let player_2 = vec![
        14, 9, 37, 47, 38, 27, 30, 24, 36, 31, 43, 42, 11, 17, 18, 10, 12, 5, 33, 25, 8, 23, 1, 13,
        49,
    ];

    let game = combat(player_1.clone(), player_2.clone());

    // Your puzzle answer was 32083.
    println!(
        "Part 1. The winning player's score: {}",
        game_score(&game.1)
    );

    // --- Part Two ---

    let game = recursive_combat(player_1, player_2);

    // Your puzzle answer was 35495.
    println!(
        "Part 2. The winning player's score: {}",
        game_score(&game.1)
    );
}
