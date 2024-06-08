#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut idx = (0..arr.len()).collect::<Vec<usize>>();
    idx.sort_by_key(|i| arr[*i]);

    let mut ans = vec![k / n; arr.len()];
    for i in 0..(k % n) {
        ans[idx[i as usize]] += 1;
    }

    println!("{}", join(&ans, "\n"));
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
    read::<String>().chars().collect::<_>()
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
