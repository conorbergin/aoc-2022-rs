use std::cmp;

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let forest : Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let width = forest.first().unwrap().len();
    let height = forest.len();

    let mut visible : Vec<u8> = vec![0;width*height];

    for i in 0..height {
        let mut l = 0;
        let mut r = 0;
        for j in 0..width {
            if forest[i][j] > l {
                l = forest[i][j];
                visible[i*height+j] = 1
            }
            if forest[i][width-j-1] > r {
                r = forest[i][width-j-1];
                visible[i*height + (width-j-1)] = 1
            }
        }
    }

    for i in 0..width {
        let mut t = 0;
        let mut b = 0;
        for j in 0..height {
            if forest[j][i] > t {
                t = forest[j][i];
                visible[j*height + i] = 1
            }
            if forest[height-j-1][i] > b {
                b = forest[height-j-1][i];
                visible[(height-j-1)*height + i] = 1
            }
        }
    }


    


    let part1 = visible.iter().filter(|&&x| x == 1).count();

    let mut part2 = 0;

    for i in 0..height {
        for j in 0..width {
            
            let tree = forest[i][j];
            
            let mut u = 0;
            let mut d = 0;
            let mut l = 0;
            let mut r = 0;
            
            while (i > u)               && (tree > forest[i-u][j] || u == 0) {u += 1}
            while (i + d < height - 1)  && (tree > forest[i+d][j] || d == 0) {d += 1}
            while (j > l)               && (tree > forest[i][j-l] || l == 0) {l += 1}
            while (j + r < width - 1)   && (tree > forest[i][j+r] || r == 0) {r += 1}

            //println!("{} {} {} {}",u,d,l,r);

            part2 = cmp::max(u*d*l*r, part2)
        }
    }

    println!{"{}\n{}",part1,part2}
}