use sscanf;
use nalgebra::{Zero};

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut c : Vec<&str> = Vec::new();
    let mut d : Vec<&str> = Vec::new();

    let mut flow_rate : Vec<u8> = Vec::new();

    for l in input.lines() {
        let p = sscanf::sscanf!(l,"Valve {str} has flow rate={u8}; tunnels lead to valves {str}").unwrap();
        c.push(p.0);
        d.push(p.2);
        flow_rate.push(p.1);
    }

    let a = flow_rate.iter().enumerate().filter(|(i,x)| **x > 0 || *i == 0).collect();

    let mut d_matrix = Zero::zero();

    

    for i in c.iter() {
        for j in 0..c.len() {
            for k in 0..c.len() {

            }
        }
    }
}