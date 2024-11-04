#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut q = vec![];
    let mut r = vec![];
    for _ in 0..n {
        let qr = readv::<i64>();
        q.push(qr[0]);
        r.push(qr[1]);
    }

    let mut ans = vec![];
    for _ in 0..read::<usize>() {
        let ask = readv::<i64>();
        let (t, d) = (ask[0] as usize - 1, ask[1]);
        ans.push(ceil_div(d - r[t], q[t]) * q[t] + r[t]);
    }

    println!("{}", join(&ans, "\n"));
}

// a.div_euclid(b) is the same as floor(a/b)
fn floor_div(a: i64, b: i64) -> i64 {
    a.div_euclid(b)
}

// Add 1 to floor(a/b) if needed
fn ceil_div(a: i64, b: i64) -> i64 {
    a.div_euclid(b) + if a.rem_euclid(b) != 0 { 1 } else { 0 }
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
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
