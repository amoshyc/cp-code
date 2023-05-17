#![allow(unused)]

use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut s = vec![];
    let mut total = 0;
    for _ in 0..n {
        let inp = reads();
        total += inp.len();
        s.push(inp);
    }
    s.sort();

    let mut t = HashSet::new();
    for _ in 0..m {
        t.insert(reads());
    }

    loop {
        let mut res: Vec<char> = vec![];
        if dfs(&s, &t, &mut res, 0, 16 - total - (n - 1)) {
            println!("{}", join(&res, ""));
            return;
        }

        if (next_permutation(&mut s).is_none()) {
            break;
        }
    }

    println!("-1");
}

fn dfs(
    s: &Vec<Vec<char>>,
    t: &HashSet<Vec<char>>,
    res: &mut Vec<char>,
    idx: usize,
    quota: usize,
) -> bool {
    res.extend(s[idx].clone());

    if idx == s.len() - 1 {
        let ok1 = (3..=16).contains(&res.len());
        let ok2 = !t.contains(res);
        let ok = ok1 && ok2;
        if !ok {
            res.truncate(res.len() - s[idx].len());
        }
        return ok1 && ok2;
    }

    for i in 0..=quota {
        res.extend(vec!['_'; i + 1]);
        if dfs(s, t, res, idx + 1, quota - i) {
            return true;
        } else {
            res.truncate(res.len() - (i + 1));
        }
    }

    res.truncate(res.len() - s[idx].len());
    false
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
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
