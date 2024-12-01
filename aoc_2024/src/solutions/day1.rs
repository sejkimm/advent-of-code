use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(input_file: &str) {
    // 입력 파일 읽기
    let lines = read_lines(input_file).expect("Could not read input file");
    let mut numbers: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(value) = line {
            numbers.push(value.parse().expect("Invalid number in input"));
        }
    }

    // 알고리즘 풀이 실행
    let result = solve(&numbers);
    println!("Day 1 Result: {}", result);
}

fn solve(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// 유틸리티 함수: 파일에서 한 줄씩 읽기
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// 테스트 로직
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_solution() {}
}
