use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut priority_sum = 0;

    for line in reader.lines() {
        let string = line.unwrap();
        let len = string.len();
        let first = &string[0..len/2];
        let second = &string[len/2..len];

        let set: HashSet<char> = HashSet::from_iter(first.chars());

        for c in second.chars() {
            if set.contains(&c) {
                priority_sum += get_priority(&c);
                break;
            }
        }
    }
    println!("Sum of the priorities is {priority_sum}");
}

fn get_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        return *c as u32 - 'A' as u32 + 27;
    }
    else {
        return *c as u32 - 'a' as u32 + 1;
    }
}
