#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (l, n1, n2) = (inp[0], inp[1], inp[2]);
    let mut v1 = vec![];
    let mut l1 = vec![];
    let mut v2 = vec![];
    let mut l2 = vec![];
    for _ in 0..n1 {
        let inp = readv::<u64>();
        v1.push(inp[0]);
        l1.push(inp[1]);
    }
    for _ in 0..n2 {
        let inp = readv::<u64>();
        v2.push(inp[0]);
        l2.push(inp[1]);
    }

    let p1 = build(&l1);
    let p2 = build(&l2);
    let mut i = 0;
    let mut j = 0;
    let mut ans: u64 = 0;
    while i < n1 && j < n2 {
        let s1 = if i == 0 { 0 } else { p1[i - 1] };
        let s2 = if j == 0 { 0 } else { p2[j - 1] };
        let e1 = p1[i];
        let e2 = p2[j];
        if v1[i] == v2[j] {
            let val = std::cmp::min(e1, e2) - std::cmp::max(s1, s2);
            ans += val as u64;
        }
        if e1 > e2 {
            j += 1;
        } else if e1 < e2 {
            i += 1;
        } else {
            i += 1;
            j += 1;
        }
    }

    println!("{}", ans);
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
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
