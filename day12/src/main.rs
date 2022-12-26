use std::collections::VecDeque;

fn search(map : &str) -> u32 {

    let m: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let mut v = vec![vec![u32::MAX; m[0].len()]; m.len()];

    let mut f = VecDeque::new();


    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 'S' {
                f.push_front((i, j));
                v[i][j] = 0;
            }
        }
    }

    fn g(c: char, d: char) -> bool {
        if d == 'S' {
            return true
        } else if c == 'E' {
            match d {
                'y' | 'z' => true,
                _ => false
            }
        } else {
            (c as u8)  <= (d as u8) + 1
        }
    }
    
    loop {
        match f.pop_back() {
            Some(x) => {
                // println!("{} {}",x.0,x.1);
                if x.0 > 0 && v[x.0 - 1][x.1] == u32::MAX && g(m[x.0-1][x.1],m[x.0][x.1]) {
                    if m[x.0-1][x.1] == 'E' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0 - 1, x.1));
                        v[x.0 - 1][x.1] = v[x.0][x.1]+1;
                    }
                }
                if x.1 > 0 && v[x.0][x.1-1] == u32::MAX && g(m[x.0][x.1-1],m[x.0][x.1]) {
                    if m[x.0][x.1-1] == 'E' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0, x.1-1));
                        v[x.0][x.1-1] = v[x.0][x.1]+1;
                    }

                }
                if x.0 < m.len() - 1 && v[x.0 + 1][x.1] == u32::MAX && g(m[x.0+1][x.1],m[x.0][x.1]) {
                    if m[x.0+1][x.1] == 'E' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0 + 1, x.1));
                        v[x.0 + 1][x.1] = v[x.0][x.1]+1;
                    }

                }
                if x.1 < m[0].len() - 1 && v[x.0][x.1+1] == u32::MAX && g(m[x.0][x.1+1],m[x.0][x.1]) {
                    if m[x.0][x.1+1] == 'E' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0, x.1+1));
                        v[x.0][x.1+1] = v[x.0][x.1]+1;
                    }
                }

                // for i in 0..m.len() {
                //     for j in 0..m[0].len() {
                //         if v[i][j] == u32::MAX {
                //             print!("M ")
                //         } else {
                //             print!("{} ",v[i][j])
                //         }
                //     }
                //     println!()
                // }

            },
            None => break
        }
    }
    0
}

fn search2(map : &str) -> u32 {

    let m: Vec<Vec<char>> = map.lines().map(|l| l.chars().collect()).collect();
    let mut v = vec![vec![u32::MAX; m[0].len()]; m.len()];

    let mut f = VecDeque::new();


    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 'E' {
                f.push_front((i, j));
                v[i][j] = 0;
            }
        }
    }

    fn g(c: char, d: char) -> bool {
        if d == 'E' {
            return true
        } else {
            (c as u8)  >= (d as u8) - 1
        }
    }
    
    loop {
        match f.pop_back() {
            Some(x) => {
                // println!("{} {}",x.0,x.1);
                if x.0 > 0 && v[x.0 - 1][x.1] == u32::MAX && g(m[x.0-1][x.1],m[x.0][x.1]) {
                    if m[x.0-1][x.1] == 'a' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0 - 1, x.1));
                        v[x.0 - 1][x.1] = v[x.0][x.1]+1;
                    }
                }
                if x.1 > 0 && v[x.0][x.1-1] == u32::MAX && g(m[x.0][x.1-1],m[x.0][x.1]) {
                    if m[x.0][x.1-1] == 'a' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0, x.1-1));
                        v[x.0][x.1-1] = v[x.0][x.1]+1;
                    }

                }
                if x.0 < m.len() - 1 && v[x.0 + 1][x.1] == u32::MAX && g(m[x.0+1][x.1],m[x.0][x.1]) {
                    if m[x.0+1][x.1] == 'a' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0 + 1, x.1));
                        v[x.0 + 1][x.1] = v[x.0][x.1]+1;
                    }

                }
                if x.1 < m[0].len() - 1 && v[x.0][x.1+1] == u32::MAX && g(m[x.0][x.1+1],m[x.0][x.1]) {
                    if m[x.0][x.1+1] == 'a' {
                        return v[x.0][x.1] + 1
                    } else {
                        f.push_front((x.0, x.1+1));
                        v[x.0][x.1+1] = v[x.0][x.1]+1;
                    }
                }

                // for i in 0..m.len() {
                //     for j in 0..m[0].len() {
                //         if v[i][j] == u32::MAX {
                //             print!("M ")
                //         } else {
                //             print!("{} ",v[i][j])
                //         }
                //     }
                //     println!()
                // }

            },
            None => break
        }
    }
    0
}

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let part1 = search(&input);

    let part2 = search2(&input);

    println! {"{}\n{}",part1,part2}
}
