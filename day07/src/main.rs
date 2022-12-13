fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut open_dirs : Vec<u32> = Vec::new();
    let mut closed_dirs = open_dirs.clone();

    for line in input.lines() {
        let mut l = line.split_whitespace();
        match l.next() {
            Some("$") => match l.next() {
                Some("cd") => match l.next() {
                    Some("..") => closed_dirs.push(open_dirs.pop().unwrap()),
                    Some(_s) => open_dirs.push(0),
                    _ => {}
                },
                _ => {}
            },
            Some(n) => {
                match n.parse::<u32>() {
                    Ok(x) => for o in open_dirs.iter_mut() {
                        *o += x;
                    },
                    _ => {}
                }
            },
            _ => {}   
        }
    }

    for _ in 0..open_dirs.len() {
        closed_dirs.push(open_dirs.pop().unwrap())
    }

    let part1 : u32 = closed_dirs.iter().filter(|&&n| n < 100000).sum();

    let space = closed_dirs.last().unwrap() - 40000000;

    let part2 = closed_dirs.iter().filter(|&&n| n > space).min().unwrap();

    println!{"{}\n{}",part1,part2}
}