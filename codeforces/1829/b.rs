#![allow(unused)]

fn solve() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if arr[i] == 1 {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < n && arr[j] == 0 {
            j += 1;
        }
        ans = ans.max(j - i);
        i = j;
    }

    println!("{}", ans);
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
