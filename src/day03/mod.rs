use crate::utils::*;
use std::collections::HashSet;

pub fn part_a(filename: Option<&String>) -> Option<usize> {
    let rounds = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };

    let mut val: u32 = 0;
    let result: u32 = rounds
        .iter()
        .map(|line| {
            let mut first_compartment = HashSet::new();

            line
                .chars()
                .enumerate()
                .find(|(i, c)| {
                    if i < &(line.len() / 2) {
                        first_compartment.insert(c.clone());
                        false
                    } else {
                        if first_compartment.contains(c) {
                            val = priority(c);
                            true
                        } else {
                            false
                        }
                    }
                });
            val
        }).sum();

    return Some(result.try_into().unwrap());
}

pub fn part_b(filename: Option<&String>) -> Option<usize> {
    let rounds = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };

    let sets: Vec<HashSet<char>> = rounds
        .iter()
        .map(|line| {
            let mut bag = HashSet::new();
            line
                .chars()
                .for_each(|c| {
                    bag.insert(c.clone());
                });
            bag
        })
        .collect();
    let mut sum = 0;
    for i in 0..(sets.len() / 3) {
        let mut hs = HashSet::new();
        let ix1 = sets[(3 * i) + 1].intersection(&sets[(3 * i) + 2]);
        for x in ix1 {
            hs.insert(*x);
        }
        let unique_item = sets[3 * i].intersection(&hs);
        let mut val = 0;

        for c in unique_item {
            val = priority(&c);
        }
        sum += val;
    }

    return Some(sum.try_into().unwrap());
}

fn priority(c: &char) -> u32 {
        let val = *c as u32;
        if val >= 97 {
            // Convert lowercase letters to 1-26 range
            val - 96
        } else {
            // Convert uppercase letters to 27-52 range
            val - 38
        }
}

fn parse_input(filename: &String) -> Vec<String> {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map(|l| l.expect("Could not parse"))
            .collect()
    } else {
        vec!()
    }
}

