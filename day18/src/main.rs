fn main() {
    let n = 26;
    let mut a = vec![vec![vec![0; n]; n]; n];

    let input = std::fs::read_to_string("input.txt").unwrap();

    for l in input.lines() {
        let mut s = l.split(',');
        a[s.next().unwrap().parse::<usize>().unwrap() + 1]
            [s.next().unwrap().parse::<usize>().unwrap() + 1]
            [s.next().unwrap().parse::<usize>().unwrap() + 1] = 1;
    }

    let mut c = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[i][j][k] == 1 {
                    c += 6
                        - a[i + 1][j][k]
                        - a[i - 1][j][k]
                        - a[i][j + 1][k]
                        - a[i][j - 1][k]
                        - a[i][j][k + 1]
                        - a[i][j][k - 1]
                }
            }
        }
    }

    let mut c2 = 0;
    let mut a = vec![vec![vec![0; n]; n]; n];

    for i in 0..n {
        let mut x = 0;
        let mut y = 0;
        for j in 0..n {
            for k in 0..n {
                if a[i][j][k + 1] == 1 || a[i][j + 1][k] == 1 {
                    x = j;
                    y = k;
                }
            }
        }

        
        loop {
            c2 += a[i + 1][x][y]
                + a[i - 1][x][y]
                + a[i][x + 1][y]
                + a[i][x - 1][y]
                + a[i][x][y + 1]
                + a[i][x][y - 1];
            
            if 
        }
    }

    let part1 = c;

    let part2 = 0;

    println!("{}\n{}", part1, part2)
}
