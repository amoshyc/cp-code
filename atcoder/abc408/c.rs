#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut diff = vec![0; n + 1];
    for _ in 0..m {
        let lr = readv::<usize>();
        let (l, r) = (lr[0] - 1, lr[1] - 1);
        diff[l] += 1;
        diff[r + 1] -= 1;
    }

    let mut cnt = vec![0; n];
    cnt[0] = diff[0];
    for i in 1..n {
        cnt[i] = cnt[i - 1] + diff[i];
    }

    let ans = cnt.iter().min().unwrap();
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
