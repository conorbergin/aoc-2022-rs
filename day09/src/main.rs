use itertools::Itertools;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn solve(input: &str, n: usize) -> usize {
    let mut rope = vec![Point { x: 0, y: 0 }; n];
    let mut visited = HashSet::new();

    for line in input.lines() {
        let (c, d) = line.split_whitespace().collect_tuple().unwrap();

        for _ in 0..d.parse().unwrap() {
            match c {
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                _ => {}
            }
            //print!("{} {}\t", rope[0].x, rope[0].y);
            for i in 1..rope.len() {
                if (rope[i - 1].x - rope[i].x).abs() == 2 && rope[i - 1].y == rope[i].y {
                    rope[i].x += (rope[i - 1].x - rope[i].x) / 2
                }
                if (rope[i - 1].y - rope[i].y).abs() == 2 && rope[i - 1].x == rope[i].x {
                    rope[i].y += (rope[i - 1].y - rope[i].y) / 2
                }

                if (rope[i - 1].x - rope[i].x).abs() == 2 && (rope[i - 1].y - rope[i].y).abs() == 1
                {
                    rope[i].x += (rope[i - 1].x - rope[i].x) / 2;
                    rope[i].y = rope[i - 1].y
                }
                if (rope[i - 1].y - rope[i].y).abs() == 2 && (rope[i - 1].x - rope[i].x).abs() == 1
                {
                    rope[i].y += (rope[i - 1].y - rope[i].y) / 2;
                    rope[i].x = rope[i - 1].x
                }

                if (rope[i - 1].x - rope[i].x).abs() == 2 && (rope[i - 1].y - rope[i].y).abs() == 2
                {
                    rope[i].x += (rope[i - 1].x - rope[i].x) / 2;
                    rope[i].y += (rope[i - 1].y - rope[i].y) / 2
                }

                //print!("{},{}\t", rope[i].x, rope[i].y);
            }
            visited.insert(rope[n - 1]);
            //println!();
        }
    }
    visited.len()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (c, d) = line.split_whitespace().collect_tuple().unwrap();
        for _ in 0..d.parse().unwrap() {
            match c {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => {}
            }

            if (head.x - tail.x).abs() == 2 && head.y == tail.y {
                tail.x += (head.x - tail.x) / 2
            }
            if (head.y - tail.y).abs() == 2 && head.x == tail.x {
                tail.y += (head.y - tail.y) / 2
            }

            if (head.x - tail.x).abs() == 2 && (head.y - tail.y).abs() == 1 {
                tail.x += (head.x - tail.x) / 2;
                tail.y = head.y
            }
            if (head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 1 {
                tail.y += (head.y - tail.y) / 2;
                tail.x = head.x
            }

            visited.insert(tail);
            //println!("{},{} {},{}",head.x,head.y,tail.x,tail.y);
        }
    }
    println! {"{}\n{}",visited.len(),solve(&input,10)}

    //for i in visited { println!("{} {}",i.x,i.y)}
}
