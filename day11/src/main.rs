
struct Monkey {
    op: String,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut inp = input.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items : Vec<Vec<u64>> = Vec::new();

    while match inp.next() {
        Some(s) => s.starts_with("Monkey"),
        None => false,
    } {
        let it: Vec<u64> = inp
            .next()
            .unwrap()
            .split(&[',', ' '][..])
            .filter(|s| !s.is_empty())
            .skip(2)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let op = inp.next().unwrap().split('=').last().unwrap().trim().to_owned();
        let test: u64 = inp
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let if_true: usize = inp
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let if_false: usize = inp
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        items.push(it);

        let m = Monkey {
            op,
            test,
            if_true,
            if_false,
        };
        monkeys.push(m);
        if inp.next() == None {
            break;
        }
    }

    let mut items2 = items.clone();
    let mut mcounter = vec![0;monkeys.len()];

    let modulo : u64 = monkeys.iter().map(|x| x.test).product();
    println!("{}",modulo);

    for _ in 0..20 {
        for i in 0..items.len()  {
            loop {
                match items[i].pop() {
                    None => break,
                    Some(x) => {
                        mcounter[i] += 1;
                        let mut n = 0;
                        if monkeys[i].op == "old * old" {
                            n = x*x
                        } else {
                            let a : u64 = monkeys[i].op.split_whitespace().last().unwrap().parse().unwrap();
                            if monkeys[i].op.starts_with("old *") {n = a*x}
                            if monkeys[i].op.starts_with("old +") {n = a+x}
                        }
                        n /= 3;
                        if n % monkeys[i].test == 0 {
                            items[monkeys[i].if_true].push(n)
                        } else {
                            items[monkeys[i].if_false].push(n)
                        }
                    }
                }

            }
        }
    }

    let mut mcounter2 = vec![0;monkeys.len()];


    for _ in 0..10_000 {
        for i in 0..items2.len()  {
            loop {
                match items2[i].pop() {
                    None => break,
                    Some(x) => {
                        let mut n = x%modulo;
                        mcounter2[i] += 1;
                        if monkeys[i].op == "old * old" {
                            n *= n 
                        } else {
                            let a : u64 = monkeys[i].op.split_whitespace().last().unwrap().parse().unwrap();
                            if monkeys[i].op.starts_with("old *") {n *= a}
                            if monkeys[i].op.starts_with("old +") {n += a}
                        }

                        if n % monkeys[i].test == 0 {
                            items2[monkeys[i].if_true].push(n)
                        } else {
                            items2[monkeys[i].if_false].push(n)
                        }
                    }
                }
            }
        }
    }

    mcounter.sort();
    mcounter.reverse();

    mcounter2.sort();
    mcounter2.reverse();

    let part1 = mcounter[0]*mcounter[1];
    let part2 = mcounter2[0]*mcounter2[1] as u64;

    println!("{}\n{}",part1,part2);

    println! {"{} {} {} {}",mcounter2[0],mcounter2[1],mcounter2[2],mcounter2[3]}
}
