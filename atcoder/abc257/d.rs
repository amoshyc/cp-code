#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut x = vec![];
    let mut y = vec![];
    let mut p = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        x.push(inp[0]);
        y.push(inp[1]);
        p.push(inp[2]);
    }

    let check = |s: i64| -> bool {
        for root in 0..n {
            let mut vis = vec![false; n];
            let mut stack = vec![];
            vis[root] = true;
            stack.push(root);
            while let Some(u) = stack.pop() {
                for v in 0..n {
                    if (x[u] - x[v]).abs() + (y[u] - y[v]).abs() <= p[u] * s {
                        if !vis[v] {
                            vis[v] = true;
                            stack.push(v);
                        }
                    }
                }
            }
            if vis.iter().all(|&x| x) {
                return true;
            }
        }
        false
    };

    // 0 0 0 1 1 1
    let mut lb = 0;
    let mut ub = 10i64.pow(10);
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if check(m) {
            ub = m;
        } else {
            lb = m;
        }
    }

    println!("{}", ub);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
