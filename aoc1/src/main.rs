use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part2() {
    if let Ok(lines) = read_lines("./input") {
        let line_numbers: Vec<_> = lines.map(|l| process_line(l)).collect();
        let numbers: Vec<_> = line_numbers
            .windows(3)
            .map(|window| window.iter().sum())
            .collect();
        let mut prev: u32 = 100000;
        let mut count: u32 = 0;

        for number in numbers {
            if number > prev {
                count += 1;
            }
            prev = number;
        }
        println!("part 2: {}", count);
    }
}

fn part1() {
    if let Ok(lines) = read_lines("./input") {
        let mut prev: u32 = 100000;
        let mut count: u32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                let cur = ip.parse::<u32>().unwrap();
                if cur > prev {
                    count += 1;
                }
                prev = cur;
            }
        }
        println!("part 1: {}", count);
    }
}

fn process_line(line: Result<String, io::Error>) -> u32 {
    line.unwrap().parse::<u32>().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
