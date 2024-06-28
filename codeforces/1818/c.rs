#![allow(unused)]

fn solve() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut f = vec![0; n];
    for i in 2..n {
        if arr[i] <= arr[i - 1] && arr[i - 1] <= arr[i - 2] {
            f[i] = 1;
        }
    }

    let pref = build(&f);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (l, r) = (inp[0] - 1, inp[1] - 1);
        
        if r - l + 1 <= 2 {
            ans.push(r - l + 1);
            continue;
        }

        let mut val = r - l + 1 - query(&pref, l, r + 1);
        if f[l] == 1 {
            val += 1;
        }
        if f[l + 1] == 1 {
            val += 1;
        }
        ans.push(val);
    }

    println!("{}", join(&ans, "\n"));
}

fn main() {
    // let tc = read::<usize>();
    let tc = 1;
    for _ in 0..tc {
        solve();
    }
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
    if i == j {
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
