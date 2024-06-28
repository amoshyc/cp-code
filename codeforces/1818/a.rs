#![allow(unused)]

fn solve() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(read::<String>());
    }

    let mut cnt = 0;
    for i in 0..n {
        if arr[i] == arr[0] {
            cnt += 1;
        }
    }

    println!("{}", cnt);
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
