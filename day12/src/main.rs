fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let map : Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut route = vec![vec![0;map[0].len()];map.len()];

    let mut boundary = Vec::new();
    
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                boundary.push((i,j))
            }
        }
    }

    loop {
        let mut tmp = Vec::new();
        for b in boundary {
            if b.0 > 0 && map[b.0-1][b.1] <= map[b.0][b.1] + 1 && route[b.0-1][b.1]
        }
        boundary = tmp;
    }



    let part1 = 0;

    let part2 = 0;

    println!{"{}\n{}",part1,part2}
}