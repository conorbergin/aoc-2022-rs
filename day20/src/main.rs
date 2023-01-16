fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut inp: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let mut ind: Vec<usize> = (0..inp.len()).collect();

    let mut inp2 = inp.clone();
    let mut ind2 = ind.clone();

    fn mix(inp: &mut Vec<i64>, ind: &mut Vec<usize>) -> () {
        for p in 0..inp.len() {
            let i = ind.iter().position(|&x| x == p).unwrap();

            let c = inp[i];
            let d = ind[i];

            let t = (c + i as i64) % (inp.len() as i64 - 1);
            let a = if t > 0 {
                t as usize
            } else {
                (t + inp.len() as i64 - 1) as usize
            };

            if a > i {
                for j in i..a {
                    inp[j] = inp[j + 1];
                    ind[j] = ind[j + 1];
                }
                inp[a] = c;
                ind[a] = d
            } else if a < i {
                for j in (a + 1..=i).rev() {
                    inp[j] = inp[j - 1];
                    ind[j] = ind[j - 1];
                }
                inp[a] = c;
                ind[a] = d
            }
        }
    }

    mix(&mut inp, &mut ind);

    let i = inp.iter().position(|&x| x == 0).unwrap();
    println!(
        "{}",
        inp[(i + 1000) % inp.len()] + inp[(i + 2000) % inp.len()] + inp[(i + 3000) % inp.len()]
    );

    for j in 0..inp2.len() {
        inp2[j] *= 811589153
    }

    for _ in 0..10 {
        mix(&mut inp2, &mut ind2)
    }

    let k = inp.iter().position(|&x| x == 0).unwrap();
    println!(
        "{}",
        inp2[(k + 1000) % inp2.len()]
            + inp2[(k + 2000) % inp2.len()]
            + inp2[(k + 3000) % inp2.len()]
    );

    // for l in inp2.iter() {
    //     println!("{}", l)
    // }
}
