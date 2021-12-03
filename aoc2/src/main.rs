use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./input") {

        let mut xy: [i32; 2]  = [0, 0];


        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                update_coordinates(&mut xy, &line);
                //line.split_ascii_whitespace();
            }
        }
        println!("forward: {}", xy[0]);
        println!("down {}", xy[1]);
        println!("final {}", xy[1] * xy[0]);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input") {

        let mut xyz: [i32; 3]  = [0, 0, 0];


        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                update_coordinates_2(&mut xyz, &line);
            }
        }
        println!("forward: {}", xyz[0]);
        println!("down {}", xyz[1]);
        println!("final {}", xyz[1] * xyz[0]);
    }
}

fn update_coordinates(xy: &mut [i32; 2], line: &String) {
    let movement = line.split_once(" ").unwrap();
    match movement {
        ("forward", x) => xy[0] += parse_int(&x),
        ("up", y) => xy[1] -= parse_int(&y),
        ("down", y) => xy[1] += parse_int(&y),
        _ => ()
    }
}

fn update_coordinates_2(xyz: &mut [i32; 3], line: &String) {
    let movement = line.split_once(" ").unwrap();
    match movement {
        ("forward", x) => {
            xyz[0] += parse_int(&x);
            xyz[1] += xyz[2] * parse_int(&x);
        },
        ("up", z) => xyz[2] -= parse_int(&z),
        ("down", z) => xyz[2] += parse_int(&z),
        _ => ()
    }
}

fn parse_int(str_num: &str) -> i32 {
    str_num.parse::<i32>().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
