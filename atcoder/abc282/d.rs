#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut color = vec![-1; n];
    let mut cnts = vec![];
    for r in 0..n {
        if color[r] == -1 {
            let mut stack = vec![r];
            let mut cnt0 = 0 as u64;
            let mut cnt1 = 0 as u64;
            color[r] = 0;
            while let Some(u) = stack.pop() {
                if color[u] == 0 {
                    cnt0 += 1;
                } else {
                    cnt1 += 1;
                }
                for &v in adj[u].iter() {
                    if color[v] == -1 {
                        color[v] = color[u] ^ 1;
                        stack.push(v);
                    }
                }
            }
            cnts.push((cnt0, cnt1));
        }
    }

    for u in 0..n {
        for &v in adj[u].iter() {
            if color[v] == color[u] {
                println!("0");
                std::process::exit(0);
            }
        }
    }

    let mut ans1 = 0;
    for &(cnt0, cnt1) in cnts.iter() {
        ans1 = ans1 + cnt0 * cnt1;
    }
    let mut ans2 = 0;
    for &(cnt0, cnt1) in cnts.iter() {
        ans2 = ans2 + (cnt0 + cnt1) * (n as u64 - cnt0 - cnt1);
    }
    ans2 /= 2;
    println!("{}", ans1 + ans2 - (m as u64));
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