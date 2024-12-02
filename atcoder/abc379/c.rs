#![allow(unused)]

use std::collections::BTreeMap;

fn main() {
    let inp = readv::<i64>();
    let (n, m) = (inp[0], inp[1] as usize);

    let x = readv::<i64>();
    let a = readv::<i64>();

    if a.iter().sum::<i64>() != n {
        println!("-1");
        return;
    }

    let mut supply = BTreeMap::new();
    for i in 0..m {
        supply.insert(x[i], a[i]);
    }

    let mut p = n;
    let mut ans = 0;
    // Keep polling the rightmost supply and update the p.
    while p >= 1 {
        if let Some((&x, &a)) = supply.range(..=p).last() {
            let k = a.min(p - x + 1);
            let q = p - k + 1;
            // move k from x to q..=p
            // total cost = (q - x) + ... + (p - x)
            ans += (q - x + p - x) * (p - q + 1) / 2;
            p = q - 1;
            supply.remove(&x);
            if k != a {
                supply.insert(x, a - k);
            }
        } else {
            println!("-1");
            return;
        }
    }

    println!("{}", ans);
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
