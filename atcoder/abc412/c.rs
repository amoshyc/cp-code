#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];

    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<usize>();

        if 2 * arr[0] >= arr[n - 1] {
            ans.push(2);
            continue;
        }

        let mut set = BTreeSet::new();
        for i in 1..n {
            set.insert((arr[i], i));
        }

        let mut cnt = 1;
        let mut ok = false;
        let mut x = arr[0];
        while let Some((y, i)) = set.range((x + 1, 0)..=(2 * x, n)).last() {
            cnt += 1;
            x = *y;
            if x >= arr[n - 1] {
                ok = true;
                ans.push(cnt);
                break;
            }
        }

        if !ok {
            ans.push(-1);
        }
    }

    println!("{}", join(&ans, "\n"));
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
