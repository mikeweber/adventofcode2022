use std::env;

mod utils;
mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let prog_num = args[1].as_str();
        let input_path = args.get(2);
        let result = match prog_num {
            "1a" => day01::part_a(input_path),
            "1b" => day01::part_b(input_path),
            "2a" => day02::part_a(input_path),
            _ => {
                println!("Could not recognize program {}", prog_num);
                None
            }
        };
        if let Some(answer) = result {
            println!("The result is {}", answer);
        } else {
            println!("No solution found for program {}", prog_num);
        }
    } else {
        println!("No program specified");
    }
}
