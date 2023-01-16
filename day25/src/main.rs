use itertools::{Itertools, EitherOrBoth};
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    fn snafu_add(a: &str, b: &str) -> String {
        let d = ['=', '-','0', '1', '2'];
        let r = a.chars()
            .rev()
            .zip_longest(b.chars().rev())
            .map(|z| match z {
                EitherOrBoth::Both(l,r) => (l,r),
                EitherOrBoth::Left(l)   => (l,'0'),
                EitherOrBoth::Right(r)  => ('0',r)
            })
            .fold(String::from('0'), |mut acc, e| {
                let c = acc.pop().unwrap();
                let i = d.iter().position(|x| *x == e.0).unwrap();
                let j = d.iter().position(|x| *x == e.1).unwrap();
                let k = d.iter().position(|x| *x == c).unwrap();
                acc.push(d[(i+j+k+1)%5]);
                acc.push(d[(i+j+k+1)/5+1]);
                acc
            });
        r.trim_end_matches('0').chars().rev().collect()
    }


    let part1 = input
        .lines()
        .fold(String::new(), |acc, e| snafu_add(&acc, e));

    let part2 = 0;

    println!("{}\n{}", part1, part2)
}
