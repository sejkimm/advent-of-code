use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <day> <part> <input_variant>");
        std::process::exit(1);
    }

    let day = &args[1];
    let part = &args[2];
    let input_variant = if args.len() > 3 { &args[3] } else { "input" };

    let input_file = match input_variant {
        "input" => format!("{}/input_{}.txt", day, part),
        "example" => format!("{}/example_input_{}.txt", day, part),
        _ => {
            eprintln!(
                "Invalid input variant: {}. Allowed values are 'input' or 'example'.",
                input_variant
            );
            std::process::exit(1);
        }
    };

    match day.as_str() {
        "day1" => match part.as_str() {
            "part1" => solutions::day1::run_part1(&input_file),
            "part2" => solutions::day1::run_part2(&input_file),
            _ => {
                eprintln!(
                    "Invalid part: {}. Allowed values are 'part1' or 'part2'.",
                    part
                );
                std::process::exit(1);
            }
        },
        "day2" => match part.as_str() {
            "part1" => solutions::day2::run_part1(&input_file),
            "part2" => solutions::day2::run_part2(&input_file),
            _ => {
                eprintln!(
                    "Invalid part: {}. Allowed values are 'part1' or 'part2'.",
                    part
                );
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Day {} is not implemented!", day);
            std::process::exit(1);
        }
    }
}
