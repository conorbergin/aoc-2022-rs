use std::collections::VecDeque;

fn main() {
    let rocks = vec![
        vec![vec![1u8, 1, 1, 1]],
        vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
        vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 0, 1]],
        vec![vec![1], vec![1], vec![1], vec![1]],
        vec![vec![1, 1], vec![1, 1]],
    ];

    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", input.len());

    {
        let mut r = rocks.iter().cycle();
        let mut g = input.trim().chars().cycle();

        let mut tower = vec![vec![0; 7]; 4000000];
        let mut h = 0;

        fn c(t: &Vec<Vec<u8>>, r: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
            for i in 0..r.len() {
                for j in 0..r[0].len() {
                    if r[i][j] == 1 && t[i + p.1][j + p.0] == 1 {
                        return true;
                    }
                }
            }
            return false;
        }

        for _ in 0..1000000 {
            let rock = r.next().unwrap();
            let mut p = (2, h + 3);
            loop {
                let n = g.next().unwrap();
                if n == '>' && p.0 + rock[0].len() < 7 && !c(&tower, rock, (p.0 + 1, p.1)) {
                    p.0 += 1
                }
                if n == '<' && p.0 > 0 && !c(&tower, rock, (p.0 - 1, p.1)) {
                    p.0 -= 1
                }
                if p.1 > 0 && !c(&tower, rock, (p.0, p.1 - 1)) {
                    p.1 -= 1
                } else {
                    break;
                }
            }

            for j in 0..rock.len() {
                for i in 0..rock[0].len() {
                    tower[p.1 + j][p.0 + i] += rock[j][i]
                }
            }
            h = std::cmp::max(p.1 + rock.len(), h);
        }

        println!("{}", h);
        let k = 77568;
        // let k = 0;
        // 155131
        for i in 0..50 {
            print!("|");
            for j in 0..7 {
                let c = if tower[k + 50 - 1 - i][j] == 1 {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c)
            }
            print!("|");
            println!()
        }
        println!("+-------+");

        'outer: for i in 1..tower.len() {
            for j in 0..i {
                if tower[j] != tower[i+j] {
                    continue 'outer
                }
            }
            println!("{}",i);
            break
        }
    }

    // {
    //     fn c(t: &VecDeque<Vec<u8>>, r: &Vec<Vec<u8>>, p: (usize, usize)) -> bool {
    //         for i in 0..r.len() {
    //             for j in 0..r[0].len() {
    //                 if r[i][j] == 1 && t[i + p.1][j + p.0] == 1 {
    //                     return true;
    //                 }
    //             }
    //         }
    //         return false;
    //     }

    //     let mut h = 0;
    //     let mut height = 0usize;

    //     let mut r = rocks.iter().cycle();
    //     let mut g = input.chars().cycle();

    //     let mut t: VecDeque<Vec<u8>> = VecDeque::new();
    //     let mut left: Option<usize> = None;
    //     let mut right: Option<usize> = None;

    //     for _ in 0..1_000_000usize {
    //         if t.len() < h+10 {
    //             for _ in 0..5 {t.push_back(vec![0, 0, 0, 0, 0, 0, 0])}
    //         }
    //         let rock = r.next().unwrap();
    //         let mut p = (2, h + 3);
    //         loop {
    //             let n = g.next().unwrap();
    //             if n == '>' && p.0 + rock[0].len() < 7 && !c(&t, rock, (p.0 + 1, p.1)) {
    //                 p.0 += 1
    //             }
    //             if n == '<' && p.0 > 0 && !c(&t, rock, (p.0 - 1, p.1)) {
    //                 p.0 -= 1
    //             }
    //             if p.1 > 0 && !c(&t, rock, (p.0, p.1 - 1)) {
    //                 p.1 -= 1
    //             } else {
    //                 break;
    //             }
    //         }

    //         for j in 0..rock.len() {
    //             for i in 0..rock[0].len() {
    //                 t[p.1 + j][p.0 + i] += rock[j][i]
    //             }
    //         }

    //         h = std::cmp::max(p.1 + rock.len(), h);

    //         if p.0 == 0 && p.1 > 0 {
    //             left = Some(p.1)
    //         }
    //         if p.0 + rock[0].len() == 6 && p.1 > 0{
    //             right = Some(p.1)
    //         }
    //         if left.is_some() && right.is_some() {
    //             let x = std::cmp::min(left.unwrap(), right.unwrap());
    //             // println!("{}",x);
    //             t.drain(0..x);
    //             h -= x;
    //             height += x;
    //             left = None;
    //             right = None;
    //         }
    //     }
    //     println!("{}", height+h);

    //     for i in 0..t.len() {
    //         for j in 0..7 {
    //             let c = if t[t.len()-i-1][j] == 1 {'#'} else {'.'};
    //             print!("{}",c)
    //         }
    //         println!()
    //     }
    // }
}
