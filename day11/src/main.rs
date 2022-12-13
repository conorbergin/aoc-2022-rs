
struct Monkey {
    op: String,
    test: i32,
    if_true: usize,
    if_false: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut inp = input.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut items : Vec<Vec<i32>> = Vec::new();
    let mut mcounter : Vec<i32> = Vec::new();

    while match inp.next() {
        Some(s) => s.starts_with("Monkey"),
        None => false,
    } {
        let it: Vec<i32> = inp
            .next()
            .unwrap()
            .split(&[',', ' '][..])
            .filter(|s| !s.is_empty())
            .skip(2)
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let op = inp.next().unwrap().split('=').last().unwrap().trim().to_owned();
        let test: i32 = inp
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
        mcounter.push(0);

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

    for _ in 0..5 {
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
                            let a : i32 = monkeys[i].op.split_whitespace().last().unwrap().parse().unwrap();
                            if monkeys[i].op.starts_with("old *") {n = a*x}
                            if monkeys[i].op.starts_with("old +") {n = a+x}
                        }
                        //n /= 3;
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

    mcounter.sort();
    mcounter.reverse();

    //let part1 : u64 = (mcounter[0] as u64)*(mcounter[1] as u64);
    let part1 = mcounter[1];
    let part2 = 0;

    println! {"{}\n{}",part1,part2}
}
