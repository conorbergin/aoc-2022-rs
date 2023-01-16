use itertools::*;

const T: bool = true;
const F: bool = false;

struct Rock {
    length: usize,
    height: usize,
    shape: [[bool; 4]; 4],
}
const ROCKS: [Rock; 5] = [
    Rock {
        length: 4,
        height: 1,
        shape: [[T, T, T, T], [F, F, F, F], [F, F, F, F], [F, F, F, F]],
    },
    Rock {
        length: 3,
        height: 3,
        shape: [[F, T, F, F], [T, T, T, F], [F, T, F, F], [F, F, F, F]],
    },
    Rock {
        length: 3,
        height: 3,
        shape: [[T, T, T, F], [F, F, T, F], [F, F, T, F], [F, F, F, F]],
    },
    Rock {
        length: 1,
        height: 4,
        shape: [[T, F, F, F], [T, F, F, F], [T, F, F, F], [T, F, F, F]],
    },
    Rock {
        length: 2,
        height: 2,
        shape: [[T, T, F, F], [T, T, F, F], [F, F, F, F], [F, F, F, F]],
    },
];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    fn gen_tower(rounds: u64, rocks: &[Rock; 5], input: &str) -> usize {
        let mut rcks = rocks.iter().cycle();
        let mut gusts = input.trim().chars().cycle();

        let mut tower = vec![[F; 7]; 40_000];
        let mut h = 0;


        let s_start = 1300;
        let mut sample = Vec::new();
        let s_length = 30;
        let mut sample_h = 0;

        for round in 0..rounds {
            let rock = rcks.next().unwrap();
            let mut x = 2usize;
            let mut y = h + 3;
            loop {
                if h + 8 > tower.len() {
                    tower.push([F;7]);
                    tower.push([F;7]);
                    tower.push([F;7]);
                    tower.push([F;7]);
                }
                let old_x = x;
                if gusts.next().unwrap() == '<' {
                    x = x.saturating_sub(1)
                } else if x + rock.length < 7 {
                    x += 1
                }

                if iproduct!(0..rock.length, 0..rock.height)
                    .any(|(i, j)| rock.shape[j][i] && tower[j + y][i + x])
                {
                    x = old_x
                }

                if y == 0
                    || iproduct!(0..rock.length, 0..rock.height)
                        .any(|(i, j)| tower[y + j - 1][x + i] && rock.shape[j][i])
                {
                    break
                } else {
                    y -= 1
                }
            }

            for (i,j) in iproduct!(0..rock.length,0..rock.height) {
                if rock.shape[j][i] {
                    tower[y + j][x + i] = rock.shape[j][i]
                }
            }

            h = h.max(y + rock.height);

            if round as usize == s_start {
                for i in (h-s_length)..h {
                    sample.push(tower[i].clone());
                    sample_h = h;
                }
            }

            if round as usize > s_start {
                if iproduct!(0..7,0..s_length).all(|(i,j)|
                    sample[j][i] == tower[j+h-s_length][i]) {
                        println!("{} {}",(round as usize - s_start),h-sample_h);
                        return (rounds / (round as usize - s_start) as u64 * (h-sample_h) as u64 ) as usize + gen_tower(rounds % (round as usize -s_start) as u64,rocks,input)
                    }
            }

        }

        // for i in (0..200).rev() {
        //     for j in 0..7 {
        //         if tower[i][j] {
        //             print!("#")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!()
        // }

        h
    }

    let part_1 = gen_tower(2022, &ROCKS, &input);
    println!("{}", part_1);

    let part_1 = gen_tower(1_000_000_000_000,&ROCKS,&input);
    println!("{}",part_1);
}