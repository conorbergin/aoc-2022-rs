fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    fn f(s:&str) -> bool {
        let v : Vec<u32> = s.split(&[',','-'][..]).map(|x| x.parse::<u32>().unwrap()).collect();
        v[0] <= v[2] && v[1] >= v[3] || v[2] <= v[0] && v[3] >= v[1] 
    }

    fn g(s:&str) -> bool {
        let v : Vec<u32> = s.split(&[',','-'][..]).map(|x| x.parse::<u32>().unwrap()).collect();
        v[0] >= v[2] && v[0] <= v[3] || v[1] >= v[2] && v[1] <= v[3]
    }

    let part1 = input.clone().lines().filter(|x| f(x)).count();

    let part2 = input.clone().lines().filter(|x| g(x) || f(x)).count();

    println!{"{}\n{}",part1,part2}
}