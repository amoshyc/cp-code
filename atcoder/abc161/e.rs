#![allow(unused)]

fn main() {
    // l[x] = the earliest day of x-th workday
    // r[x] = the latest day of x-th workday
    // x-th workday is bound to work if and only if l[x] = r[x]
    // Find l[x], r[x] via greedy.

    let inp = readv::<usize>();
    let (n, k, c) = (inp[0], inp[1], inp[2]);
    let s = reads();

    let mut l = vec![];
    for i in 0..n {
        if s[i] == 'o' {
            if l.is_empty() || i > *l.last().unwrap() + c {
                l.push(i);
            }
        }
    }

    let mut r = vec![];
    for i in (0..n).rev() {
        if s[i] == 'o' {
            if r.is_empty() || i + c < *r.last().unwrap() {
                r.push(i);
            }
        }
    }
    r.reverse();

    if l.len() != k || r.len() != k {
        println!("");
        return;
    }

    let ans: Vec<usize> = (0..l.len())
        .filter(|&x| l[x] == r[x])
        .map(|x| l[x] + 1)
        .collect();
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
