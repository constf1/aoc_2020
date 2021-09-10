use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    pub fn from_str(s: &str) -> Self {
        let a_start = "(contains";
        let a_end = ")";
        let a_pos = s
            .find(a_start)
            .zip(s.rfind(a_end))
            .expect("fail to find allergens");

        Self {
            ingredients: s[..a_pos.0]
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
            allergens: s[a_pos.0 + a_start.len()..a_pos.1]
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

fn read_input() -> std::io::Result<Vec<Food>> {
    let file = File::open("./data/day-21.txt")?;

    let mut buf = Vec::new();
    for line in BufReader::new(file).lines() {
        let s = line?;
        let t = s.trim();
        if !t.is_empty() {
            buf.push(Food::from_str(&t));
        }
    }

    Ok(buf)
}

pub fn run() {
    println!("--- Day 21: Allergen Assessment ---");

    let foods = read_input().expect("fail to read input");

    let mut ingredients = HashSet::new();
    let mut allergens = HashSet::new();

    for food in &foods {
        ingredients.extend(&food.ingredients);
        allergens.extend(&food.allergens);
    }

    // Determine which ingredients cannot possibly contain any of the allergens in your list.
    // How many times do any of those ingredients appear?
    let mut inert_ingredients = HashSet::new();
    let mut i_count = 0;

    // Each allergen is found in exactly one ingredient.
    // Each ingredient contains zero or one allergen.
    for ingredient in &ingredients {
        // Determine which ingredients can't possibly contain any of the allergens in any food in your list.
        let mut a_count = 0;
        for allergen in &allergens {
            for food in &foods {
                if !food.ingredients.contains(ingredient) && food.allergens.contains(allergen) {
                    // ingredients cannot have any allergen
                    a_count += 1;
                    break;
                }
            }
        }

        if a_count == allergens.len() {
            i_count += foods
                .iter()
                .filter(|&food| food.ingredients.contains(ingredient))
                .count();
            inert_ingredients.insert(*ingredient);
        }
    }

    // Your puzzle answer was 2211.
    println!("Part 1. Inert ingredient count: {}", i_count);

    // --- Part Two ---

    let mut allergens: HashMap<&str, Vec<&str>> = HashMap::new();
    for food in &foods {
        let dangerous_ingredients: HashSet<&str> = food
            .ingredients
            .iter()
            .filter(|i| !inert_ingredients.contains(i))
            .map(String::as_str)
            .collect();
        for allergen in &food.allergens {
            if let Some(ingredient_set) = allergens.get_mut(allergen.as_str()) {
                ingredient_set.retain(|&i| dangerous_ingredients.contains(i));
            } else {
                allergens.insert(allergen, dangerous_ingredients.iter().copied().collect());
            }
        }
    }

    // You should have enough information to figure out which ingredient contains which allergen.
    let mut dangers: Vec<(&str, &str)> = Vec::with_capacity(allergens.len());
    while !allergens.is_empty() {
        for (k, v) in allergens.iter().filter(|(_, v)| v.len() == 1) {
            dangers.push((v[0], k));
        }
        allergens.retain(|_, v| v.len() > 1);

        for v in allergens.values_mut() {
            v.retain(|&k| dangers.iter().find(|d| d.0 == k).is_none());
        }
    }
    dangers.sort_by(|a, b| a.1.cmp(b.1));

    // Arrange the ingredients alphabetically by their allergen and separate them by commas
    // to produce your canonical dangerous ingredient list.
    let list = dangers
        .into_iter()
        .map(|d| d.0)
        .collect::<Vec<&str>>()
        .join(",");

    // Your puzzle answer was 'vv,nlxsmb,rnbhjk,bvnkk,ttxvphb,qmkz,trmzkcfg,jpvz'.
    println!("Part 2. Your canonical dangerous ingredient list: {}", list);
}
