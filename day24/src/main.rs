use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut b = input
        .lines()
        .skip(1)
        .map(|s| {
            s.chars()
                .skip(1)
                .take_while(|c| *c != '#')
                .map(|c| match c {
                    '.' => [0, 0, 0, 0],
                    '^' => [1, 0, 0, 0],
                    'v' => [0, 1, 0, 0],
                    '<' => [0, 0, 1, 0],
                    '>' => [0, 0, 0, 1],
                    _ => !unreachable!(),
                })
                .collect_vec()
        })
        .take_while(|v| !v.is_empty())
        .collect_vec();

    let d = b.len();
    let w = b[0].len();
    // not the cleverest metric
    let steps = (d + w)*10;

    let mut m = Vec::new();
    for _ in 0..steps {
        let mut b_next = b.clone();
        for i in 0..d {
            for j in 0..w {
                b_next[i][j] = [
                    b[(i + 1) % d][j][0],
                    b[(i + d - 1) % d][j][1],
                    b[i][(j + 1) % w][2],
                    b[i][(j + w - 1) % w][3],
                ]
            }
        }
        b = b_next;
        m.push(
            b.iter()
                .map(|r| r.iter().map(|e| e == &[0, 0, 0, 0]).collect_vec())
                .collect_vec(),
        );
    }

    fn walk(m: &Vec<Vec<Vec<bool>>>, t: usize, to: (usize, usize), from: (usize, usize)) -> usize {
        let mut frontier = HashSet::from([from]);

        for k in t..m.len() {
            let mut f_next = HashSet::new();
            if frontier.is_empty() && m[k][from.0][from.1] {
                frontier = HashSet::from([from]);
            }
            for f in frontier.iter() {
                if m[k][f.0][f.1] {
                    f_next.insert(f.clone());
                }
                if f.0 > 0 && m[k][f.0 - 1][f.1] {
                    f_next.insert((f.0 - 1, f.1));
                }
                if f.1 > 0 && m[k][f.0][f.1 - 1] {
                    f_next.insert((f.0, f.1 - 1));
                }
                if f.0 < m[0].len() - 1 && m[k][f.0 + 1][f.1] {
                    f_next.insert((f.0 + 1, f.1));
                }
                if f.1 < m[0][0].len() - 1 && m[k][f.0][f.1 + 1] {
                    f_next.insert((f.0, f.1 + 1));
                }
            }
            frontier = f_next;
            // print!("{}:\t",k+1);
            // for f in frontier.iter() {
            //     print!("{} {}\t",f.0,f.1)
            // }
            // println!();
            if frontier.contains(&to) {
                return k + 2;
            }
        }
        0
    }

    let part_1 = walk(&m,1,(d-1,w-1),(0,0));
    println!("{}", part_1);
    let part_2 = walk(&m,walk(&m,part_1+1,(0,0),(d-1,w-1))+1,(d-1,w-1),(0,0));
    println!("{}", part_2)
}
