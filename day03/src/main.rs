use itertools::Itertools;

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    fn priority(c : char) -> u32 {
        let p = c as u32;
        match p {
            97..=140 => p - 96,
            65..=90 => p - 64 + 26,
            _ => 0
        }

    }

    fn rucksack(s: &str) -> u32 {
        let (s1,s2) = s.split_at(s.len()/2);
        for c in s1.chars() {
            if s2.contains(c) {return priority(c)}
        }
        0
    }

    let part1 : u32 = input.clone().lines().map(|s| rucksack(s)).sum();

    let mut part2 : u32 = 0;

    for (a,b,c) in input.clone().lines().tuples() {
        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                part2 += priority(ch);
                break
            }
        }
    }

    println!{"{}\n{}",part1,part2};
}