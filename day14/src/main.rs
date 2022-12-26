use std::cmp::{max, min};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let walls: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|x| {
                    let mut y = x.split(',');
                    (
                        y.next().unwrap().parse::<usize>().unwrap(),
                        y.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let depth = walls.iter().flatten().map(|&x| x.1).max().unwrap() + 3;

    let x_max = 500 + depth;
    let x_min = 500 - depth;
    

    println!("depth : {}",depth);

    let mut cave = vec![vec!['.'; depth]; x_max-x_min];

    for wall in walls.iter() {
        let mut prev = wall[0];
        for curr in wall.iter().skip(1) {
            if prev.0 == curr.0 {
                for i in min(prev.1, curr.1)..=max(prev.1, curr.1) {
                    cave[prev.0 - x_min][i] = '#'
                }
            } else {
                for j in min(prev.0, curr.0)..=max(prev.0, curr.0) {
                    cave[j - x_min][prev.1] = '#'
                }
            }
            prev = *curr
        }
    }

    let mut cave2 = cave.clone();
    for i in 0..cave2.len() {
        cave2[i][depth-1] = '#'
    }

    let mut counter = 0;
    'outer: loop {
        counter += 1;
        let mut sand = (500-x_min,0);
        for _ in 0..depth-1 {
            if cave[sand.0][sand.1+1] == '.' {
                sand = (sand.0,sand.1+1)
            } else if cave[sand.0-1][sand.1+1] == '.' {
                sand = (sand.0-1,sand.1+1)
            } else if cave[sand.0+1][sand.1+1] == '.' {
                sand = (sand.0+1,sand.1+1)
            } else {
                cave[sand.0][sand.1] = 'o';
                continue 'outer
            }
        }
        break
    }

    let mut counter2 = 0;
    'outer: loop {
        counter2 += 1;
        let mut sand = (500-x_min,0);
        loop {
            if cave2[sand.0][sand.1+1] == '.' {
                sand = (sand.0,sand.1+1)
            } else if cave2[sand.0-1][sand.1+1] == '.' {
                sand = (sand.0-1,sand.1+1)
            } else if cave2[sand.0+1][sand.1+1] == '.' {
                sand = (sand.0+1,sand.1+1)
            } else {
                cave2[sand.0][sand.1] = 'o';
                if sand == (500-x_min,0) {break 'outer}
                continue 'outer
            }
        }
    }

    for j in 0..depth {
        for i in 0..cave.len() {
            print!("{}", cave2[i][j])
        }
        println!()
    }

    println!("{} {}",counter-1,counter2)
}
