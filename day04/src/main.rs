use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut fully_containing_pairs = 0;
    let mut overlapping_pairs = 0;

    for line in reader.lines() {
        let string = line.unwrap();
        let mut ranges = string.splitn(2, ',');

        let (first_start, first_end) = parse_range(ranges.next().unwrap());
        let (second_start, second_end) = parse_range(ranges.next().unwrap());

        // part 1
        if first_start <= second_start && first_end >= second_end || second_start <= first_start && second_end >= first_end {
            fully_containing_pairs += 1;
        }

        // part 2
        if first_start <= second_start && first_end >= second_start || second_start <= first_start && second_end >= first_start {
            overlapping_pairs += 1;
        }
    }
    println!("Number of fully containing pairs is {fully_containing_pairs}");
    println!("Number of overlapping pairs is {overlapping_pairs}");

}

fn parse_range(string: &str) -> (u32, u32) {
    let mut splitter = string.splitn(2, '-');
    let first = splitter.next().unwrap().parse().unwrap();
    let second = splitter.next().unwrap().parse().unwrap();
    (first, second)
}
