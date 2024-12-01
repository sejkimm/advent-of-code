use std::env;

mod solutions;

fn main() {
    // 명령줄 인자 처리
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        std::process::exit(1);
    }

    let day = &args[1];
    match day.as_str() {
        "day1" => solutions::day1::run("day1/input.txt"),
        _ => eprintln!("Day {} is not implemented!", day),
    }
}
