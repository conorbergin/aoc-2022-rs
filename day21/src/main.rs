use sscanf;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    fn f(input: &str, root: &str) -> String {
        let mut code = vec![root];
        let mut out = root.to_string();

        while let Some(c) = code.pop() {
            let x = input.lines().find(|s| s.starts_with(c)).unwrap();
            let y = x.split(':').skip(1).next().unwrap().trim();
            let z = format!("({})", &y);
            out = out.replace(c, &z);

            if let Ok((a, _, c)) = sscanf::sscanf!(&y, "{str} {str} {str}") {
                code.push(a);
                code.push(c);
            }
        }
        out
    }

    let s = f(&input, "root");

    let sol = caldyn::eval(&s, None).unwrap();


    println!("{}", sol);

    let inp2 = input.lines().fold(String::new(), |mut result, line| {
        if line.starts_with("humn") {
            result.push_str("humn: h\n");
        } else {
            result.push_str(line);
            result.push_str("\n");
        }
        result
    });

    let rt = inp2.lines().find(|s| s.starts_with("root")).unwrap();

    let (l,_,r) = sscanf::sscanf!(rt, "root: {str} {str} {str}").unwrap();

    let left = f(&inp2,r);
    let right = f(&inp2,l);

    let lhs = caldyn::eval(&left, None).unwrap();
    println!("{}",lhs);



    let mut ctx =  caldyn::Context::new();

    ctx.set("h",3582317956029f64); // manual gradient descent unfortunately, very lazy
    let x = caldyn::eval(&right,&ctx).unwrap();
    println!("{}",(x-lhs).abs() as i64)
}
