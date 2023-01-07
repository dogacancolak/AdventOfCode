use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let file = File::open("src/input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut priority_sum = 0;
    let mut badge_priority_sum = 0;
    let mut common_items = HashSet::new();

    for (i, line) in reader.lines().enumerate() {
        let string = line.unwrap();

        // Part 1
        let items: HashSet<char> = HashSet::from_iter(string.chars());
        // the first elf in each group
        if (i + 1) % 3 == 1 {
            common_items = items;
        }
        else {
            common_items = common_items.into_iter().filter(|c| items.contains(c)).collect();
        }
        // the third elf in each group
        if (i + 1) % 3 == 0 {
            let c = common_items.drain().next().unwrap();
            badge_priority_sum += get_priority(c);
        }

        // Part 2
        let len = string.len();
        let first = &string[0..len/2];
        let second = &string[len/2..len];

        let set: HashSet<char> = HashSet::from_iter(first.chars());

        for c in second.chars() {
            if set.contains(&c) {
                priority_sum += get_priority(c);
                break;
            }
        }
    }
    println!("Sum of the priorities is {priority_sum}");
    println!("Sum of the badge priorities is {badge_priority_sum}");

}

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        return c as u32 - 'A' as u32 + 27;
    }
    else {
        return c as u32 - 'a' as u32 + 1;
    }
}
