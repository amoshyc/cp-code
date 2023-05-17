#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, w) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let mut p = std::collections::HashSet::new();

    for i in 0..n {
        p.insert(arr[i]);
        for j in (i + 1)..n {
            p.insert(arr[i] + arr[j]);
            for k in (j + 1)..n {
                p.insert(arr[i] + arr[j] + arr[k]);
            } 
        }
    }

    let ans = (1..=w).filter(|&i| p.contains(&i)).count();
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
