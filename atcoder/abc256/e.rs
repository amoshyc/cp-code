#![allow(unused)]


fn main() {
    let n = read::<usize>();
    let x = readv::<usize>();
    let c = readv::<i64>();

    let x = x.iter().map(|&x| x - 1).collect::<Vec<_>>();
    
    let mut rem = vec![false; n];
    let mut ind = vec![0; n];
    for u in 0..n {
        ind[x[u]] += 1;
    }

    let mut que = std::collections::VecDeque::new();
    for u in 0..n {
        if ind[u] == 0 {
            que.push_back(u);
            rem[u] = true;
        }
    }
    while let Some(u) = que.pop_front() {
        ind[x[u]] -= 1;
        if ind[x[u]] == 0 {
            que.push_back(x[u]);
            rem[x[u]] = true;
        }
    }

    let mut ans = 0;
    for u in 0..n {
        if rem[u] {
            continue;
        }
        rem[u] = true;
        let mut val = c[u];
        let mut v = x[u];
        while v != u {
            rem[v] = true;
            val = val.min(c[v]);
            v = x[v];
        }
        ans += val;
    }

    println!("{}", ans);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
