use crate::utils::*;
use std::collections::HashMap;

pub fn part_a(filename: Option<&String>) -> Option<usize> {
    let rounds = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };

    let mut point_map: HashMap<String, i32> = HashMap::new();
    point_map.insert(String::from("A X"), 4);
    point_map.insert(String::from("A Y"), 8);
    point_map.insert(String::from("A Z"), 3);
    point_map.insert(String::from("B X"), 1);
    point_map.insert(String::from("B Y"), 5);
    point_map.insert(String::from("B Z"), 9);
    point_map.insert(String::from("C X"), 7);
    point_map.insert(String::from("C Y"), 2);
    point_map.insert(String::from("C Z"), 6);

    let result = rounds
        .iter()
        .fold(0, |s, r| s + point_map[&r.to_string()]);
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

    let mut point_map: HashMap<String, i32> = HashMap::new();
    point_map.insert(String::from("A X"), 3);
    point_map.insert(String::from("A Y"), 4);
    point_map.insert(String::from("A Z"), 8);
    point_map.insert(String::from("B X"), 1);
    point_map.insert(String::from("B Y"), 5);
    point_map.insert(String::from("B Z"), 9);
    point_map.insert(String::from("C X"), 2);
    point_map.insert(String::from("C Y"), 6);
    point_map.insert(String::from("C Z"), 7);

    let result = rounds
        .iter()
        .fold(0, |s, r| s + point_map[&r.to_string()]);
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


/*
part 1
 XYZ
A483
B159
C726

part 2
 XYZ
A348
B159
C267
*/
