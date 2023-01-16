use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let m_tmp = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut map = vec![vec!['.'; m_tmp[0].len() * 3]; m_tmp.len() * 3];
    for i in 0..m_tmp.len() {
        for j in 0..m_tmp[0].len() {
            map[i + m_tmp.len()][j + m_tmp[0].len()] = m_tmp[i][j]
        }
    }

    let mut dirs = ['^', 'v', '<', '>'];
    let mut i_count = 0;
    loop {
        i_count += 1;

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == '#'
                    && (map[i + 1][j - 1] == '#'
                        || map[i + 1][j] == '#'
                        || map[i + 1][j + 1] == '#'
                        || map[i][j + 1] == '#'
                        || map[i - 1][j + 1] == '#'
                        || map[i - 1][j] == '#'
                        || map[i - 1][j - 1] == '#'
                        || map[i][j - 1] == '#')
                {
                    for d in dirs {
                        let (p, q, r) = match d {
                            '^' => ((i - 1, j), (i - 1, j - 1), (i - 1, j + 1)),
                            'v' => ((i + 1, j), (i + 1, j - 1), (i + 1, j + 1)),
                            '<' => ((i, j - 1), (i - 1, j - 1), (i + 1, j - 1)),
                            '>' => ((i, j + 1), (i - 1, j + 1), (i + 1, j + 1)),
                            _ => !unreachable!(),
                        };

                        if map[p.0][p.1] != '#' && map[q.0][q.1] != '#' && map[r.0][r.1] != '#' {
                            match map[p.0][p.1] {
                                '.' => map[p.0][p.1] = d,
                                '^' => map[p.0][p.1] = '.',
                                'v' => map[p.0][p.1] = '.',
                                '<' => map[p.0][p.1] = '.',
                                '>' => map[p.0][p.1] = '.',
                                _ => !unreachable!(),
                            }

                            break;
                        }
                    }
                }
            }
        }

        if !map
            .iter()
            .flatten()
            .any(|c| matches!(*c, '^' | 'v' | '<' | '>'))
        {
            break;
        }

        for i in 0..map.len() {
            for j in 0..map[0].len() {
                match map[i][j] {
                    '^' => {
                        map[i][j] = '#';
                        map[i + 1][j] = '.'
                    }
                    'v' => {
                        map[i][j] = '#';
                        map[i - 1][j] = '.'
                    }
                    '<' => {
                        map[i][j] = '#';
                        map[i][j + 1] = '.'
                    }
                    '>' => {
                        map[i][j] = '#';
                        map[i][j - 1] = '.'
                    }
                    _ => {}
                }
            }
        }

        dirs = [dirs[1], dirs[2], dirs[3], dirs[0]];

        if i_count == 10 {
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = 0;
            let mut max_y = 0;

            let mut count = 0;

            for i in 0..map.len() {
                for j in 0..map[0].len() {
                    if map[i][j] == '#' {
                        count += 1;
                        min_x = if (j as i32) < min_x { j as i32 } else { min_x };
                        min_y = if (i as i32) < min_y { i as i32 } else { min_y };
                        max_x = if (j as i32) > max_x { j as i32 } else { max_x };
                        max_y = if (i as i32) > max_y { i as i32 } else { max_y }
                    }
                }
            }

            println!("{}", (max_x - min_x + 1) * (max_y - min_y + 1) - count);
        }
    }

    println!("{}", i_count)
}
