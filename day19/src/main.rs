fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let b_ps : Vec<[u16;6]> = input.lines().map(|s| {
        let p = sscanf::sscanf!(s,"Blueprint {u16}: Each ore robot costs {u16} ore. Each clay robot costs {u16} ore. Each obsidian robot costs {u16} ore and {u16} clay. Each geode robot costs {u16} ore and {u16} obsidian.").unwrap();
        [p.1,p.2,p.3,p.4,p.5,p.6]
    }).collect();

    fn dfs(t: u16, bp: &[u16; 6]) -> u16 {
        
        fn f(t: u16, bp: &[u16; 6], m: [u16; 4], r: [u16; 4], max: &[u16; 3], best: &mut u16) {
            
            if r[3] + m[3] * t + (t - 1) * t / 2 < *best {
                return;
            }

            let mut recursed = false;
            // ore robot
            for j in 1..t {
                if (j - 1) * m[0] + r[0] >= bp[0] && m[0] <= max[0] {
                    let nr = [
                        r[0] + m[0] * j - bp[0],
                        r[1] + m[1] * j,
                        r[2] + m[2] * j,
                        r[3] + m[3] * j,
                    ];
                    f(t - j, bp, [m[0] + 1, m[1], m[2], m[3]], nr, max, best);
                    recursed = true;
                    break;
                }
            }
            // clay robot
            for j in 1..t {
                if (j - 1) * m[0] + r[0] >= bp[1] && m[1] <= max[1] {
                    let nr = [
                        r[0] + m[0] * j - bp[1],
                        r[1] + m[1] * j,
                        r[2] + m[2] * j,
                        r[3] + m[3] * j,
                    ];
                    f(t - j, bp, [m[0], m[1] + 1, m[2], m[3]], nr, max, best);
                    recursed = true;
                    break;
                }
            }
            // obsidian robot
            for j in 1..t {
                if (j - 1) * m[0] + r[0] >= bp[2]
                    && (j - 1) * m[1] + r[1] >= bp[3]
                    && m[2] <= max[2]
                {
                    let nr = [
                        r[0] + m[0] * j - bp[2],
                        r[1] + m[1] * j - bp[3],
                        r[2] + m[2] * j,
                        r[3] + m[3] * j,
                    ];
                    f(t - j, bp, [m[0], m[1], m[2] + 1, m[3]], nr, max, best);
                    recursed = true;
                    break;
                }
            }
            // geode robot
            for j in 1..t {
                if (j - 1) * m[0] + r[0] >= bp[4] && (j - 1) * m[2] + r[2] >= bp[5] {
                    let nr = [
                        r[0] + m[0] * j - bp[4],
                        r[1] + m[1] * j,
                        r[2] + m[2] * j - bp[5],
                        r[3] + m[3] * j,
                    ];
                    f(t - j, bp, [m[0], m[1], m[2], m[3] + 1], nr, max, best);
                    recursed = true;
                    break;
                }
            }

            if !recursed {
                *best = (r[3] + m[3] * t).max(*best);
                return;
            }
        }

        let mut best = 0;
        let max = [
            [bp[0], bp[1], bp[2], bp[4]].into_iter().max().unwrap(),
            bp[3],
            bp[5],
        ];
        f(t, bp, [1, 0, 0, 0], [0, 0, 0, 0], &max, &mut best);
        best
    }

    let part_1: u16 = b_ps
        .iter()
        .enumerate()
        .map(|(i, b)| (i as u16 + 1) * dfs(24, b))
        .sum();
    println!("{} ", part_1);

    let part_2: u16 = b_ps.iter().take(3).map(|b| dfs(32, b)).product();
    println!("{}", part_2)
}
