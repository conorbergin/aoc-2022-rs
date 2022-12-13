fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();



    let part1 : u32 = input.lines().map( |x|
        match x {
            "A X" => 1+3,
            "A Y" => 2+6,
            "A Z" => 3+0,
            "B X" => 1+0,
            "B Y" => 2+3,
            "B Z" => 3+6,
            "C X" => 1+6,
            "C Y" => 2+0,
            "C Z" => 3+3,
            _ => 0
        }).sum();

    let part2 : u32 = input.lines().map( |x|
        match x {
            "A X" => 3+0,
            "A Y" => 1+3,
            "A Z" => 2+6,
            "B X" => 1+0,
            "B Y" => 2+3,
            "B Z" => 3+6,
            "C X" => 2+0,
            "C Y" => 3+3,
            "C Z" => 1+6,
            _ => 0
        }).sum();    

    println!("{}\n{}", part1, part2);
    

}
