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
                            val = *c as u32;
                            if val >= 97 {
                                // Convert lowercase letters to 1-26 range
                                val -= 96;
                            } else {
                                // Convert uppercase letters to 27-52 range
                                val -= 38;
                            }
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

fn parse_input(filename: &String) -> Vec<String> {
    if let Ok(lines) = read_lines(filename) {
        lines
            .map(|l| l.expect("Could not parse"))
            .collect()
    } else {
        vec!()
    }
}

