use itertools::Itertools;
use nalgebra::{Vector2,Matrix2,DMatrix};

const R: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
const L: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

fn ret(p: Vector2<i32>, d: Vector2<i32>) -> i32 {
    let f = match (d.x, d.y) {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => !unreachable!(),
    };

    1000 * (p.x + 1) + 4 * (p.y + 1) + f
}

fn walk_2d(map: &DMatrix<char>, step: Vec<i32>, turn: Vec<char>) -> i32 {
    let mut p = Vector2::new(0i32, 0);
    let mut d = Vector2::new(0i32, 1);

    while map[(p.x as usize, p.y as usize)] != '#' {
        p += d
    }

    for (i, &s) in step.iter().enumerate() {
        for _ in 0..s {
            match map.get(((p.x + d.x) as usize, (p.y + d.y) as usize)) {
                Some(' ') | None => {
                    let mut q = p;
                    while matches!(
                        map.get(((q.x - d.x) as usize, (q.y - d.y) as usize)),
                        Some('.') | Some('#')
                    ) {
                        q -= d
                    }
                    if map[(q.x as usize, q.y as usize)] == '.' {
                        p = q
                    }
                }
                Some('.') => p += d,
                Some('#') => break,
                Some(_) => unreachable!(),
            }
        }

        match turn.get(i) {
            Some('L') => d = L * d,
            Some('R') => d = R * d,
            _ => {}
        }
    }

    ret(p, d)
}

fn walk_cube(map: &DMatrix<char>, step: Vec<i32>, turn: Vec<char>) -> i32 {
    let mut p = Vector2::new(0i32, 0);
    let mut d = Vector2::new(0i32, 1);

    // only implemented for 3x4 and 4x3 nets
    let scale = std::cmp::max(map.nrows(), map.ncols()) as i32 / 4;

    while map[(p.x as usize, p.y as usize)] == ' ' {
        p += d
    }

    for (i, s) in step.iter().enumerate() {
        for _ in 0..*s {
            match map.get(((p.x + d.x) as usize, (p.y + d.y) as usize)) {
                Some(' ') | None => {
                    let (mut q, mut e) = (p, d);
                    // cube wrapping algorithm
                    e = R * e;
                    let dis = match (e.x, e.y) {
                        (1,0) =>  p.x % scale + 1,
                        (0,1) =>  p.y % scale + 1,
                        (-1,0) => scale - p.x % scale,
                        (0,-1) => scale - p.y % scale,
                        _ => !unreachable!()
                    };

                    q = q + (scale-dis) * e;


                    let mut stack = "".to_string();

                    loop {
                        let (l, s, r) = (
                            (q + e) + L * e * scale,
                            q + e * scale,
                            q + R * e * (scale - 1),
                        );
                        if matches!(map.get((l.x as usize, l.y as usize)), Some('.') | Some('#')) {
                            q = l;
                            e = L * e;
                            stack.push('L')
                        } else if matches!( map.get((s.x as usize, s.y as usize)), Some('.') | Some('#')) {
                            q = s;
                            stack.push('S')
                        } else {
                            q = r;
                            e = R * e;
                            stack.push('R')
                        }

                        if stack.ends_with("SLR") || stack.ends_with("RLS") {
                            stack.drain(stack.len()-3..stack.len());
                            stack.push('L')
                        }

                        if stack.ends_with("RLR") {
                            stack.drain(stack.len()-3..stack.len());
                            stack.push('S')
                        }

                        if stack == "L".to_string() {
                            break
                        }

                    }

                    q = q - (dis - 1) * e;
                    e = R * e;

                    if map[(q.x as usize, q.y as usize)] == '.' {
                        p = q;
                        d = e
                    }
                }
                Some('.') => p += d,
                Some('#') => break,
                Some(_) => unreachable!(),
            }
        }

        match turn.get(i) {
            Some('L') => d = L * d,
            Some('R') => d = R * d,
            _ => {}
        }
    }

    ret(p, d)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (m, r) = input.split("\n\n").collect_tuple().unwrap();

    let n_rows = m.lines().count();
    let n_cols = m.lines().map(|l| l.chars().count()).max().unwrap();

    let map = DMatrix::from_iterator(
        n_cols,
        n_rows,
        m.lines()
            .map(|l| l.chars().pad_using(n_cols, |_| ' '))
            .flatten(),
    )
    .transpose();

    let steps = r
        .split(&['L', 'R'][..])
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
    let turns = r.chars().filter(|&c| c == 'L' || c == 'R').collect_vec();

    let part_1 = walk_2d(&map, steps.clone(), turns.clone());
    println!("{}", part_1);

    let part_1 = walk_cube(&map, steps.clone(), turns.clone());
    println!("{}", part_1);
}
