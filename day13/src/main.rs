use std::cmp::Ordering;

fn mysplit(s: &str) -> Vec<String> {
    if s == "[]" {return vec![String::new()]}
    let mut l = s.chars();
    let mut ret: Vec<String> = Vec::new();
    match l.next() {
        Some(c) if c == '[' => 
        loop {
            match l.next() {
                None => return ret,
                Some(c) => match c {
                    '[' => {
                        let mut s = String::from(c);
                        let mut n = 1;
                        while n != 0 {
                            let t = l.next().unwrap();
                            if t == '[' {
                                n += 1
                            }
                            if t == ']' {
                                n -= 1
                            }
                            s.push(t)
                        }
                        ret.push(s)
                    }
                    '0'..='9' | 'A' => ret.push(c.to_string()),
                    _ => {}
                },
            }
        },
        _ => return vec![s.to_string()],
    }
}

enum Order {
    Equal,
    Left(String),
    Right(String),
}

fn f(left: &str, right: &str) -> Order {
    // println!("-> left: {}\n-> right: {}",left,right);
    if left == right {
        Order::Equal 
    } else if left.is_empty() {
        Order::Left("Left empty".to_string())
    } else if right.is_empty() {
        Order::Right("Right empty".to_string())
    } else if left.starts_with('[') || right.starts_with('[') {
        let l = mysplit(left);
        let r = mysplit(right);
        for i in 0..std::cmp::min(l.len(), r.len()) {
            match f(&l[i], &r[i]) {
                Order::Equal => {}
                other => return other,
            }
        }
        if left.len() <= right.len() {
            Order::Left("Ran out of items".to_string())
        } else {
            Order::Right("Ran out of items".to_string())
        }
    } else {
        if (left.chars().next().unwrap() as u8) < (right.chars().next().unwrap()) as u8 {
            Order::Left(left.clone().to_string() + " is smaller than " + &right)
        } else {
            Order::Right(left.clone().to_string() + " is greater than " + &right)
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("minput.txt")
        .unwrap()
        .replace("10", "A");

    let part1 : usize = input.trim().split("\n\n").enumerate().map(|(i,x)| {
            let mut l = x.split('\n');
            (i,f(l.next().unwrap(),l.next().unwrap()))
        }).filter(|(_,y)| matches!(y,Order::Left(_))).map(|(i,_)| i+1).sum();

    let mut lines : Vec<&str> = input.split_whitespace().collect();

    lines.push("[[2]]");
    lines.push("[[6]]");

    lines.sort_by(|a,b| if matches!(f(a,b),Order::Left(_)) {Ordering::Less} else {Ordering::Greater});

    let part2 : usize = lines.iter().enumerate().filter(|(_,&s)| s == "[[2]]" || s == "[[6]]").map(|(i,_)| i+1).product();

    // println!("{}",mysplit(tmp.next().unwrap())[0]);
    println!("{}", part1);
    println!("{}", part2);
}
