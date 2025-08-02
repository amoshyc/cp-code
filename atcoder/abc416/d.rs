#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let tc = read::<usize>();
    let mut res = vec![];
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (n, m) = (inp[0], inp[1]);
        let mut arr_a = readv::<usize>();
        let arr_b = readv::<usize>();

        let mut set = BTreeSet::new();
        for i in 0..n {
            set.insert((arr_b[i] % m, i));
        }

        let mut ans = 0;
        arr_a.sort();
        arr_a.reverse();
        for a in arr_a {
            let a = a % m;
            if let Some(&(b, i)) = set.range((m - a, 0)..).next() {
                ans += (a + b) % m;
                set.remove(&(b, i));
            } else {
                let (b, i) = *set.first().unwrap();
                ans += (a + b) % m;
                set.remove(&(b, i));
            }
        }
        res.push(ans);
    }

    println!("{}", join(&res, "\n"));
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
