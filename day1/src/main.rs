use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp;

fn main() {
    most_calories();
    top_3_most_calories();
}

fn most_calories() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let file = BufReader::new(file);
    let mut curr_elf = 0;
    let mut max = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        if line.is_empty() {
            max = cmp::max(max, curr_elf);
            curr_elf = 0;
        }
        else {
            let calory = line.parse::<i32>().unwrap();
            curr_elf += calory;
        }
    }
    println!("The max is {}", max);
}

fn top_3_most_calories() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let file = BufReader::new(file);
    let mut curr_elf = 0;
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        if line.is_empty() {
            if curr_elf > first {
                third = second;
                second = first;
                first = curr_elf;
            }
            else if curr_elf > second {
                third = second;
                second = curr_elf;
            }
            else if curr_elf > third {
                third = curr_elf;
            }
            curr_elf = 0;
        }
        else {
            let calory = line.parse::<i32>().unwrap();
            curr_elf += calory;
        }
    }

    println!("The sum of the max 3 is {}", first + second + third);
}
