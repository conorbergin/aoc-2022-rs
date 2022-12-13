use itertools::Itertools;



fn f(s : &[u8]) -> bool {
    for i in 0..(s.len()-1) {
        for j in (i+1)..s.len() {
            if s[i] == s[j] { return false }
        }
    }
    true
}

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let part1 = input.chars().tuple_windows().position(|(a,b,c,d)| a != b && a != c && a != d && b != c && b != d && c != d ).unwrap() + 4;

    let part2 = input.as_bytes().windows(14).position(|x| f(x)).unwrap() + 14;

    println!{"{}\n{}",part1,part2}
}