
fn mix(mut inp: Vec<i64>, n: usize) -> i64 {
    let mut ind = (0..inp.len()).collect::<Vec<_>>();
    for _ in 0..n {
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
    let i = inp.iter().position(|&x| x == 0).unwrap();
    inp[(i + 1000) % inp.len()] + inp[(i + 2000) % inp.len()] + inp[(i + 3000) % inp.len()]
}

fn main() {
    let input = std::fs::read_to_string("minput.txt").unwrap();

    let part_1 = mix(
        input
            .lines()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
        1,
    );
    println!("{}", part_1);

    let part_2 = mix(
        input
            .lines()
            .map(|x| x.parse::<i64>().unwrap() * 811589153)
            .collect::<Vec<_>>(),
        10,
    );
    println!("{}", part_2)
}
