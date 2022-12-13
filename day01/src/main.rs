fn main() {

    fn sum_str(s : &str) -> u32 {
        s.split_whitespace().map(|x| x.parse::<u32>().unwrap()).sum()
    }


    let input = std::fs::read_to_string("input.txt").unwrap();
    let totals = input.split("\n\n").map( |x| sum_str(x) );

    let part1 = totals.clone().max().unwrap();

    let mut top_three : Vec<u32> = vec![0;3];

    for t in totals {
        top_three.sort();
        if t > top_three[0] {top_three[0] = t}
    }

    let part2 : u32 = top_three.iter().sum();


    println!("{}\n{}", part1, part2);

}
