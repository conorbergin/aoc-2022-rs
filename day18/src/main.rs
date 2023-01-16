use std::collections::VecDeque;

use sscanf;

fn main() {
    let n = 26;
    let mut a = vec![vec![vec![0u8; n]; n]; n];

    let input = std::fs::read_to_string("input.txt").unwrap();

    for l in input.lines() {
        let parsed = sscanf::sscanf!(l,"{usize},{usize},{usize}").unwrap();
        a[parsed.0 + 1][parsed.1 + 1][parsed.2 + 1] = 1;
    }

    let mut c = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[i][j][k] == 1 {
                    c += 6
                        - a[i + 1][j][k] as u32
                        - a[i - 1][j][k] as u32
                        - a[i][j + 1][k] as u32
                        - a[i][j - 1][k] as u32
                        - a[i][j][k + 1] as u32
                        - a[i][j][k - 1] as u32
                }
            }
        }
    }

    println!("{}",c);

    let mut q = VecDeque::from([(0,0,0)]);

    while let Some(p) = q.pop_back() {
        if p.0 > 0     && a[p.0-1][p.1][p.2] == 0 {a[p.0-1][p.1][p.2] = u8::MAX; q.push_front((p.0-1,p.1,p.2))}
        if p.1 > 0     && a[p.0][p.1-1][p.2] == 0 {a[p.0][p.1-1][p.2] = u8::MAX; q.push_front((p.0,p.1-1,p.2))}
        if p.2 > 0     && a[p.0][p.1][p.2-1] == 0 {a[p.0][p.1][p.2-1] = u8::MAX; q.push_front((p.0,p.1,p.2-1))}
        if p.0 < n - 1 && a[p.0+1][p.1][p.2] == 0 {a[p.0+1][p.1][p.2] = u8::MAX; q.push_front((p.0+1,p.1,p.2))}
        if p.1 < n - 1 && a[p.0][p.1+1][p.2] == 0 {a[p.0][p.1+1][p.2] = u8::MAX; q.push_front((p.0,p.1+1,p.2))}
        if p.2 < n - 1 && a[p.0][p.1][p.2+1] == 0 {a[p.0][p.1][p.2+1] = u8::MAX; q.push_front((p.0,p.1,p.2+1))}
    }


    let mut c2 = 0;

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[i][j][k] == u8::MAX {
                    if i > 0     && a[i-1][j][k] == 1 {c2 += 1}
                    if j > 0     && a[i][j-1][k] == 1 {c2 += 1}
                    if k > 0     && a[i][j][k-1] == 1 {c2 += 1}
                    if i < n - 1 && a[i+1][j][k] == 1 {c2 += 1}
                    if j < n - 1 && a[i][j+1][k] == 1 {c2 += 1}
                    if k < n - 1 && a[i][j][k+1] == 1 {c2 += 1}
                }
            }
        }
    }

    println!("{}",c2)
}
