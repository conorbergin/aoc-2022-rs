use std::collections::HashMap;

use itertools::iproduct;
use sscanf;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut id = Vec::new();
    let mut ed: Vec<Vec<&str>> = Vec::new();
    let mut fr: Vec<u32> = Vec::new();

    for line in input.lines() {
        let parsed = line.split_whitespace().collect::<Vec<&str>>();
        id.push(parsed[1]);
        fr.push(sscanf::sscanf!(parsed[4], "rate={u32};").unwrap());
        ed.push(parsed[9..].iter().map(|s| &s[0..2]).collect());
    }

    let n = id.len();
    let aa = id.iter().position(|&s| s == "AA").unwrap();

    let mut dist: Vec<Vec<u32>> = ed
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let idx: Vec<usize> = e
                .iter()
                .map(|f| id.iter().position(|y| y == f).unwrap())
                .collect();
            (0..n)
                .map(|x| {
                    if idx.contains(&x) {
                        1
                    } else if x == i {
                        0
                    } else {
                        u32::MAX / 2
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect();

    for _ in 0..2 {
        for (i, j, k) in iproduct!(0..n, 0..n, 0..n) {
            dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
        }
    }

    let mut aax = 0;
    let mut frx = Vec::new();
    for i in 0..n {
        if fr[i] > 0 {
            frx.push(fr[i])
        }
        if i == aa {
            aax = frx.len();
            frx.push(0);
        }
    }
    let distx: Vec<Vec<u32>> = dist
        .iter()
        .enumerate()
        .filter(|(i, _)| fr[*i] > 0 || *i == aa)
        .map(|(_, r)| {
            r.iter()
                .enumerate()
                .filter(|(j, _)| fr[*j] > 0 || *j == aa)
                .map(|(_, &c)| c)
                .collect()
        })
        .collect();

    fn g(
        t: u32,
        r: u32,
        M: &Vec<Vec<u32>>,
        fr: &Vec<u32>,
        cur: usize,
        visited: u32,
        ans: &mut HashMap<u32,u32>,
    ) {
        if let Some(old_r) = ans.get(&visited) {
            if old_r > &r {
                return
            }
        }
        ans.insert(visited, r);
        for i in (0..M.len()).filter(|&x| x != cur && 1 << x & visited == 0 && M[cur][x] < t - 1) {
            let new_t = t - M[cur][i] - 1;
            let new_r = r + fr[i] * new_t;
            let new_v = visited | 1 << i;
            g(new_t, new_r, M, fr, i, new_v, ans)
        }
    }

    let mut h = HashMap::new();
    g(30, 0, &distx, &frx, aax, 0, &mut h);
    println!("{}", h.values().max().unwrap());

    let mut h2 = HashMap::new();
    g(26, 0, &distx, &frx, aax, 0, &mut h2);
    println!("{}",iproduct!(h2.iter(),h2.iter()).filter(|(a,b)| a.0 & b.0 == 0).map(|(a,b)| *a.1 + *b.1).max().unwrap())

}
