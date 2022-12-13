use std::{collections::VecDeque, fs};

struct Printer {
    counter : i32,
    output : String
}

impl Printer {
    fn new() -> Self {
        Printer {counter : 0, output : String::new()}
    }

    fn draw(&mut self, r: i32) {
        
        if self.counter >= r - 1 && self.counter <= r + 1 {
            self.output.push('#')
        } else {
            self.output.push('.')
        }
        if self.counter == 39 {
            self.output.push('\n');
            self.counter = 0
        } else {
            self.counter += 1
        }
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines = input.lines();
    let mut register: i32 = 1;
    let mut r = vec![register];
    let mut q = VecDeque::new();

    let mut out = Printer::new();

    loop {

        match lines.next() {
            Some(s) => {
                out.draw(register);
                r.push(register);
                q.push_back(0);
                let mut l = s.split_whitespace();
                if l.next().unwrap() == "addx" {
                    q.push_back(l.next().unwrap().parse().unwrap())
                }
                register += q.pop_front().unwrap();
            },
            None => match q.pop_front() {
                Some(x) => {
                    out.draw(register);
                    r.push(register);
                    register += x;
                },
                None => break
            }
        }
    }

    let part1: i32 = r
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .take(6)
        .map(|(i, &x)| x * (i as i32))
        .sum();

    println!("{}", part1);
    println!("{}",out.output)
}
