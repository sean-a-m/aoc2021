use bit_field::BitField;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./input") {
        let numbers: Vec<_> = lines
            .map(|line| process_to_number(&line.unwrap()))
            .collect();
        let mut counts: [i32; 12] = [0; 12];
        for number in numbers {
            update_count(&mut counts, number);
        }

        let (epsilon, gamma) = process_counts_2(&counts, 12);
        let mut gamma2 = !epsilon;
        gamma2.set_bits(12..=15, 0b0000);
        println!("gamma2: {}", gamma2);
        let power: u32 = gamma as u32 * epsilon as u32;
        println!("epsilon: {}", epsilon);
        println!("gamma: {}", gamma);
        println!("power: {}", gamma as u32 * epsilon as u32)
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input") {
        let numbers: Vec<_> = lines
            .map(|line| process_to_number(&line.unwrap()))
            .collect();

        let ox = find_ox(&numbers);
        let co2 = find_co2(&numbers);

        println!("ox: {}", ox);
        println!("co2: {}", co2);
        println!("life rating: {}", ox as u32 * co2 as u32);
    }
}

fn find_ox(numbers: &Vec<u16>) -> u16 {
    let mut valid: Vec<u16> = numbers.clone();

    for n in 0..12 {
        let most_common = get_most_common_at_n(&valid, 11-n, valid.len());
        valid = valid
            .iter()
            .filter(|number| number.get_bit(11-n) == most_common)
            .cloned()
            .collect();
        println!("at iteration {} there are {} numbers with a {}", n, valid.len(), most_common);
        if valid.len() == 1 {
            break;
        }
    }

    return *valid.get(0).unwrap();
}

fn find_co2(numbers: &Vec<u16>) -> u16 {
    let mut valid: Vec<u16> = numbers.clone();

    for n in 0..12 {
        let least_common = get_least_common_at_n(&valid, 11-n, valid.len());
        valid = valid
            .iter()
            .filter(|number| number.get_bit(11-n) == least_common)
            .cloned()
            .collect();
        println!("at iteration {} there are {} numbers with a {}", n, valid.len(), least_common);
        if valid.len() == 1 {
            break;
        }
    }

    return *valid.get(0).unwrap();
}


fn filter_gamma(numbers: &Vec<u16>, mask: u16) -> u16 {
    let mut valid: Vec<u16> = numbers.clone();

    for n in 0..12 {
        valid = valid
            .iter()
            .filter(|number| number.get_bit(n) == mask.get_bit(n))
            .cloned()
            .collect();
        println!("at iteration {} there are {} valid numbers with a {}", n, valid.len(), mask.get_bit(n));
        if valid.len() == 1 {
            break;
        }
    }

    return *valid.get(0).unwrap();
}

fn update_count(counts: &mut [i32; 12], number: u16) {
    for n in 0..12 {
        match number.get_bit(n) {
            true => counts[11 - n] += 1,
            _ => (),
        }
    }
}

fn process_counts(counts: &[i32; 12], total: i32) -> (u16, u16) {
    let mut epsilon: u16 = 0b0000_0000_0000_0000;
    let mut gamma: u16 = 0b0000_0000_0000_0000;
    for n in 0..=11 {
        epsilon <<= 1;
        gamma <<= 1;
        if counts[n] >= (total / 2) {
            epsilon |= 0b0000_0000_0000_0001;
        } else {
            gamma |= 0b0000_0000_0000_0001;
        }
    }
    return (epsilon, gamma);
}

fn process_counts_2(counts: &[i32; 12], total: i32) -> (u16, u16) {
    let mut epsilon: u16 = 0b0000_0000_0000_0000;
    let mut gamma: u16 = 0b0000_0000_0000_0000;
    for n in 0..=11 {
        if counts[n] >= (total / 2) {
            epsilon.set_bit(11 - n, true);
        }
        if counts[n] <= (total / 2) {
            gamma.set_bit(11 - n, true);
        }
    }
    return (epsilon, gamma);
}

fn get_most_common_at_n(numbers: &Vec<u16>, place: usize, total: usize) -> bool {
    let mut count_1 = 0;
    let mut count_0 = 0;
    for number in numbers {
        if number.get_bit(place) == true {
            count_1 += 1;
        } else {
            count_0 +=1
        }
    }
    println!("{} numbers with 1 at {} against {}", count_1, place, count_0);

    if count_1 >= count_0{
        return true;
    }
    return false;
}

fn get_least_common_at_n(numbers: &Vec<u16>, place: usize, total: usize) -> bool {
    let mut count_1 = 0;
    let mut count_0 = 0;
    for number in numbers {
        if number.get_bit(place) == true {
            count_1 += 1;
        } else {
            count_0+=1;
        }
    }
    println!("{} numbers with 1 at {} against {}", count_1, place, count_0);
    if count_1 >= count_0 {
        return false;
    }
    return true;
}


fn process_to_number(line: &String) -> u16 {
    let mut bitfield: u16 = 0b0000_0000_0000_0000;
    for (pos, val) in line.chars().enumerate() {
        match val {
            '1' => {
                bitfield.set_bit(11 - pos, true);
            }
            _ => (),
        };
    }
    return bitfield;
}

fn process_to_number_2(line: &String) -> u16 {
    let mut bitfield: u16 = 0b0000_0000_0000_0000;
    for (pos, val) in line.chars().enumerate() {
        match val {
            '1' => {
                bitfield.set_bit(11 - pos, true);
            }
            _ => (),
        };
    }
    return bitfield;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
