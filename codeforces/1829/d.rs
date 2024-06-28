#![allow(unused)]

fn solve() {
    let inp = readv::<u64>();
    let n = inp[0];
    let m = inp[1];

    if m > n {
        println!("No");
        return;
    }

    if m == n {
        println!("Yes");
        return;
    }

    let mut s = std::collections::HashSet::new();
    s.insert(n);
    let mut cnt = 1;
    let mut x = n;
    while x % 3 == 0 {
        x = x / 3;
        cnt += 1;
        for i in 0..cnt {
            s.insert(x * (1 << i));
        }
    }
    if s.contains(&m) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
    }
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
