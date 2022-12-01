use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<usize> {
    let lines = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };
    return Some(*lines.iter().max().unwrap());
}

pub fn part_b(filename: Option<&String>) -> Option<usize> {
    let mut lines = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };
    lines.sort();
    return Some(lines[lines.len() - 1] + lines[lines.len() - 2] + lines[lines.len() - 3])
}

fn parse_input(filename: &String) -> Vec<usize> {
    let mut calories = vec!();
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            if let Ok(r) = line {
                if r == "" {
                    calories.push(sum);
                    sum = 0;
                } else {
                    sum += r.parse::<usize>().unwrap();
                }
            }
        }
        calories.push(sum);
    }

    return calories;
}
