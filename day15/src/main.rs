use itertools::*;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let data: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|s| {
            sscanf::sscanf!(
                s,
                "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
            )
            .unwrap()
        })
        .collect();

    let intercept = 2_000_000;
    let part_1 = data
        .iter()
        .filter_map(|(sx, sy, bx, by)| {
            let o = (bx - sx).abs() + (by - sy).abs() - (intercept - sy).abs();
            if o > 0 {
                if *sy == intercept {
                    Some(vec![(sx - o, sx + o)])
                } else {
                    Some(vec![(sx - o, sx - 1), (sx + 1, sx + o)])
                }
            } else {
                None
            }
        })
        .flatten()
        .fold(Vec::new(), |acc: Vec<(i32, i32)>, mut e| {
            let mut out = Vec::new();
            for a in acc {
                if a.0 >= e.1 || a.1 <= e.0 {
                    out.push(a)
                } else {
                    e = (e.0.min(a.0), e.1.max(a.1))
                }
            }
            out.push(e);
            out
        })
        .iter()
        .fold(0, |acc, e| acc + 1 + (e.1 - e.0));

    println!("{}", part_1);


    
    
    fn f(x0:i32,y0:i32,x1:i32,y1:i32,data:&Vec<(i32,i32,i32,i32)>) -> Option<u64> {


        let split = if x1-x0 >= y1 - y0 {
            [(x0,y0,(x1+x0)/2,y1),((x1+x0)/2 + 1,y0,x1,y1)]
        } else {
            [(x0,y0,x1,(y0+y1)/2),(x0,(y0+y1)/2 + 1,x1,y1)]
        };

        for (a,b,c,d) in split {

            if data.iter().all(|(sx,sy,bx,by)| {
                let r = (bx-sx).abs() + (by-sy).abs();
                iproduct!([a,c],[b,d]).any(|(x,y)| {
                    (x-sx).abs() + (y-sy).abs() > r
                })
            }) {
                if a == c && b == d {
                    return Some(a as u64*4_000_000 + b as u64)
                } else {
                    match f(a,b,c,d,data) {
                        Some(x) => return Some(x),
                        None => {}
                    }
                }
            }
        }            
        None
    }

    let part_2 = f(0,0,4_000_000,4_000_000,&data).unwrap();
    // let part_2 = f(0,0,20,20,&data).unwrap();
    println!("{}",part_2);
}
