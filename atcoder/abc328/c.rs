#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let s = reads();

    let mut arr = vec![0; n];
    for i in 1..n {
        if s[i - 1] == s[i] {
            arr[i] = 1;
        }
    }

    let pref = build(&arr);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (l, r) = (inp[0] - 1, inp[1] - 1);
        ans.push(query(&pref, l + 1, r + 1));
    }

    println!("{}", join(&ans, "\n"));
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
    if i >= j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
