#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    let mut pref = vec![];
    for r in 0..k {
        let x = arr
            .iter()
            .enumerate()
            .map(|(i, &x)| if i % k == r { arr[i] } else { 0 })
            .collect::<Vec<_>>();
        pref.push(build(&x));
    }

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (a, b) = (inp[0] - 1, inp[1] - 1);

        let mut sum = vec![0; k];
        for r in 0..k {
            sum[r] = query(&pref[r], a, b + 1);
        }
        sum.sort();
        sum.dedup();

        if sum.len() == 1 {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
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
fn query<T: Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
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
