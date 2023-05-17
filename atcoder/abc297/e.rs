#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<u64>();

    let mut cands = std::collections::BTreeSet::new();
    for i in 0..n {
        cands.insert(arr[i]);
    }

    let mut cnt = 0;
    while let Some(u) = cands.range(0..).next().cloned() {
        cands.remove(&u);
        cnt += 1;
        if cnt == k {
            println!("{}", u);
            return;
        }
        for i in 0..n {
            cands.insert(u + arr[i]);
        }
    }
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
