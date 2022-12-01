use crate::utils::*;

pub fn part_a(filename: Option<&String>) -> Option<usize> {
    let lines = match filename {
        Some(path) => parse_input(path),
        None => {
            println!("Please specify which file you'd like to run");
            return None;
        }
    };
    return find_value(&lines);
}

fn find_value(lines: &Vec<usize>) -> Option<usize> {
    let mut sum = 0;
    for (i, x) in lines.iter().enumerate() {
        sum += x * (i + 1);
    }

    return Some(sum);
}

fn parse_input(filename: &String) -> Vec<usize> {
    let mut result: Vec<usize> = vec!();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(r) = line {
                let val = r.parse::<usize>().unwrap();
                result.push(val);
            }
        }
    }

    return result;
}
