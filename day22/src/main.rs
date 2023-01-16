use itertools::Itertools;

type Pt = (i32, i32);

fn main() {
    let input = std::fs::read_to_string("minput.txt").unwrap();

    let (m, r) = input.split("\n\n").collect_tuple().unwrap();
    let map = m.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let steps = r
        .split(&['L', 'R'][..])
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec();
    let turns = r.chars().filter(|&c| c == 'L' || c == 'R').collect_vec();

    fn walk(
        map: &Vec<Vec<char>>,
        step: Vec<i32>,
        turn: Vec<char>,
        check_next: &dyn Fn(&Vec<Vec<char>>, Pt, Pt) -> Option<Pt>,
    ) {
        let mut turn = turn.iter();

        let mut d = (0, 1);
        let mut p = (0, 0);

        while map[p.0 as usize][p.1 as usize] == ' ' {
            p = (p.0, p.1 + 1)
        }

        for s in step {
            for _ in 0..s {
                match check_next(map, p, d) {
                    None => break,
                    Some(q) => p = q,
                }
            }

            if let Some(c) = turn.next() {
                match c {
                    'L' => d = (-d.1, d.0), // (1,0) -> (0,1) -> (-1,0) -> (0,-1)
                    'R' => d = (d.1, -d.0), // (1,0) -> (0,-1) -> (-1,0) -> (0,1)
                    _ => {}
                }
            }
        }
        let f = match d {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => 0,
        };

        println!("{}", 1000 * (p.0+1) + 4 * (p.1+1) + f);
    }

    fn step_2d(m: &Vec<Vec<char>>, p: Pt, d: Pt) -> Option<Pt> {
        let mut q = ((p.0+d.0+m.len() as i32) % (m.len() as i32),(p.1+d.1+m[0].len() as i32) % m[0].len() as i32);
        match m[q.0 as usize ][q.1 as usize] {
            ' ' => {
                while m[q.0 as usize][q.1 as usize] == ' '{
                    q = ((q.0+d.0+m.len() as i32) % (m.len() as i32),(q.1+d.1+m[0].len() as i32) % m[0].len() as i32);
                }
                if m[q.0 as usize][q.1 as usize] == '.' {
                    Some(q)
                } else {
                    None
                }
            }
            '.' => Some(q),
            '#' | _ => None,

        }
    }

    walk(&map, steps.clone(), turns.clone(), &step_2d);



    fn step_cube(m: &Vec<Vec<char>>, p:Pt,d:Pt) -> Option<Pt> {
        match m[(p.0 + d.0) as usize][(p.1+d.1) as usize] {
            ' ' => {
                Some(p)
            },
            '#' => None,
            _ => Some((p.0+d.0,p.1+d.1)),
        }
    }

    fn normalise_cube(m: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (w,h) = (m[0].len(), m.len());
        
    }

    // for i in 0..map.len() {
    //     for j in 0..map[0].len() {
    //         print!("{}", map[i][j])
    //     }
    //     println!()
    // }


    // walk(&map, steps, turns, &step_cube);
}
