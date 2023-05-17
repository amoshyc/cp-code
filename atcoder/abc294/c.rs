#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let a = readv::<i32>();
    let b = readv::<i32>();
    let mut c = a.clone();
    c.extend(&b);
    c.sort();

    let ans = a
        .iter()
        .map(|&x| partition_point(&c, |&y| y < x) + 1)
        .collect::<Vec<_>>();
    println!("{}", join(&ans, " "));
    
    let ans = b
        .iter()
        .map(|&x| partition_point(&c, |&y| y < x) + 1)
        .collect::<Vec<_>>();
    println!("{}", join(&ans, " "));
}

// arr.partition_point is added at 1.52.0
// 1 1 1 0 0 0
//       ^
fn partition_point<T, P: FnMut(&T) -> bool>(arr: &[T], mut pred: P) -> usize {
    arr.binary_search_by(|x| {
        if pred(x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
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
