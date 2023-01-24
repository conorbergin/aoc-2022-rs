use itertools::*;

const T: bool = true;
const F: bool = false;

const TOWER_MAX: usize = 100_000;

const SAMPLE_ROUND : usize = 1_000;
const SAMPLE_LENGTH : usize = 50;


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


fn gen_tower(rounds: usize, rocks: &[Rock; 5], input: &str) -> usize {
    let mut rks = rocks.iter().cycle();
    let mut gusts = input.trim().chars().cycle();

    let mut tower = [[F; 7]; TOWER_MAX];
    let mut h = 0;


    let mut sample_h = 0;
    let mut sample = [[F;7];SAMPLE_LENGTH];

    for round in 0..rounds {

        let rock = rks.next().unwrap();
        let mut x = 2usize;
        let mut y = h + 3;

        loop {
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
                break;
            } else {
                y -= 1
            }
        }

        for (i, j) in iproduct!(0..rock.length, 0..rock.height) {
            if rock.shape[j][i] {
                tower[y + j][x + i] = rock.shape[j][i]
            }
        }

        h = h.max(y + rock.height);



        if round == SAMPLE_ROUND {
            sample_h = h;
            for i in 0..SAMPLE_LENGTH {
                for j in 0..7 {
                    sample[i][j] = tower[i+h-SAMPLE_LENGTH][j]
                }
            }

        }

        if round > SAMPLE_ROUND {
            if iproduct!(0..7, 0..SAMPLE_LENGTH).all(|(i, j)| tower[j+h-SAMPLE_LENGTH][i] == sample[j][i])
            {
                println!("Sample:");
                for l in (0..SAMPLE_LENGTH).rev() {
                    for c in 0..7 {
                        print!("{}",if tower[h-SAMPLE_LENGTH+l][c] {"#"} else {"."})
                    }
                    println!()
                }
                println!();
                println!("Match:");
                for l in (0..SAMPLE_LENGTH).rev() {
                    for c in 0..7 {
                        print!("{}",if tower[h-SAMPLE_LENGTH+l][c] {"#"} else {"."})
                    }
                    println!()
                }
                println!();
                let repeat_h = h - sample_h;
                let repeat_r = round - SAMPLE_ROUND;
                println!("rounds repeat: {}", repeat_r);
                println!("height repeat: {}", repeat_h);
                println!("{}, {}",rounds / repeat_r, rounds % repeat_r);
                return ((rounds / repeat_r)-1) * (repeat_h)
                    + gen_tower(rounds % repeat_r + repeat_r, rocks, input);
            }
        }
    }
    h
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let part_1 = gen_tower(2022, &ROCKS, &input);
    println!("{}", part_1);

    let part_1 = gen_tower(1_000_000_000_000, &ROCKS, &input);
    println!("{}", part_1);
}
