use std::{cmp, net::Incoming};



fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.strip_prefix("Sensor at x=")
                .unwrap()
                .split(": closest beacon is at x=")
                .map(|a| {
                    a.split(", y=")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect()
        })
        .collect();

    let min_x = data.iter().map(|v| cmp::min(v[0],v[2])).min().unwrap();
    let min_y = data.iter().map(|v| cmp::min(v[1],v[3])).min().unwrap();
    let max_x = data.iter().map(|v| cmp::max(v[0],v[2])).max().unwrap();
    let max_y = data.iter().map(|v| cmp::max(v[1],v[3])).max().unwrap();

    let pad_x = data.iter().map(|v| (v[0]-v[2]).abs()).max().unwrap();
    let pad_y = data.iter().map(|v| (v[1]-v[3]).abs()).max().unwrap();


    fn in_cone(p :(i32,i32), c : &Vec<i32>) -> bool {
        if (p.0 == c[0] && p.1 == c[1]) || (p.0 == c[2] && p.1 == c[3]) {
            return false
        } else if (p.0-c[0]).abs() + (p.1-c[1]).abs() <= (c[2]-c[0]).abs() + (c[3]-c[1]).abs() {
            return true
        } else {
            return false
        }
    }

    let width : usize = (max_x - min_x + 1 + 2*pad_x).try_into().unwrap();
    let depth : usize = (max_y - min_y + 1 + 2*pad_y).try_into().unwrap();

    let offset_x = min_x - pad_x;
    let offset_y = min_y + pad_y;

    println!("{} {} {} {}    {} {} ",min_x,min_y,max_x,max_y,offset_x,offset_y);


   //let part1 = (0..width).filter(|&x| in_cone(((x as i32+offset_x),2_000_000),&data)).count();

    println!("{} {}",0,u64::MAX);

    fn find_beacon(cones: &Vec<Vec<i32>>) -> Option<u64> {
        for c in cones {
            
        }
    }

    let part2 = find_beacon((0,0,4_000_000,4_000_000), &data).unwrap();

    println!("{}",part2);
    



}
