#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<i64>();
    let (n, m, k) = (inp[0] as usize, inp[1] as usize, inp[2]);
    let arr = readv::<i64>();

    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by_key(|i| Reverse(arr[*i]));
    let mut val = vec![0; n];
    for i in 0..n {
        val[i] = arr[indices[i]];
    }
    let pref = build(&val);
    let rest = k - val.iter().sum::<i64>();

    let mut ans = vec![0; n];
    for i in 0..n {
        let ok = |a: i64| -> bool {
            let p = val.partition_point(|x| *x > val[i] + a);
            let c = find_c(&pref, p, i, val[i] + a + 1, rest - a);
            p + c < m
        };

        // 0 0 0 1 1 1
        let mut lb = 0;
        let mut ub = rest;
        if ok(lb) {
            ans[indices[i]] = 0;
            continue;
        }
        if !ok(ub) {
            ans[indices[i]] = -1;
            continue;
        }
        while ub - lb > 1 {
            let a = (lb + ub) / 2;
            if ok(a) {
                ub = a;
            } else {
                lb = a;
            }
        }
        ans[indices[i]] = ub;
    }

    println!("{}", join(&ans, " "));
}

fn find_c(pref: &Vec<i64>, p: usize, i: usize, target: i64, rest: i64) -> usize {
    let ok = |q: usize| -> bool {
        let mut need = target * (q - p + 1) as i64;
        need -= query(&pref, p, q + 1);
        if p <= i && i <= q {
            need -= target;
            need += query(&pref, i, i + 1);
        }
        rest >= need
    };
    // 1 1 1 0 0 0
    let mut lb = p;
    let mut ub = pref.len() - 1;

    if !ok(lb) {
        return 0;
    }
    if ok(ub) {
        let mut ans = ub - p + 1;
        if p <= i && i <= ub {
            ans -= 1;
        }
        return ans;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    let mut ans = lb - p + 1;
    if p <= i && i <= ub {
        ans -= 1;
    }
    ans
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
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
