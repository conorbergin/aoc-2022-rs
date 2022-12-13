fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (header, body) = input.split_once("\n\n").unwrap();
    
    let mut h = header.lines().rev();

    let n = h.next().unwrap().trim().chars().last().unwrap() as usize - 48;
    let mut stack : Vec<Vec<char>> = vec![Vec::new();n];

    for l in h {
        let b = l.as_bytes();
        for i in 0..n {
            let c = b[i*4 + 1] as char;
            if c != ' ' {
                stack[i].push(c)
            }
        }
    }

    let mut stack2 = stack.clone();


    for l in body.lines() {
        let v : Vec<usize> = l.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect();
        
        for _ in 0..v[0] {
            let t =stack[v[1]-1].pop().unwrap();
            stack[v[2]-1].push(t)
        }

        let mut tmp : Vec<char> = Vec::new();
        for _ in 0..v[0] {
            let t = stack2[v[1]-1].pop().unwrap();
            tmp.push(t)
        }
        tmp.reverse();
        stack2[v[2]-1].append(&mut tmp);

    }

    for i in 0..n {
        print!("{}",stack[i].last().unwrap())
    }
    println!();

    for i in 0..n {
        print!("{}",stack2[i].last().unwrap())
    }
    println!();

}