#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let a = readv::<usize>();
    let b = readv::<usize>();

    let mut s = std::collections::BTreeSet::new();
    for i in 0..n {
        s.insert((a[i], i));
    }

    for i in 0..m {
        if let Some((x, y)) = s.range((b[i], 0)..=(b[i], n)).next() {
            s.remove(&(*x, *y));
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
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
