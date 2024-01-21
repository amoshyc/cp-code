#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (w, h) = (inp[0], inp[1]);
    let n = read::<usize>();

    let mut pos = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        pos.push((inp[0], inp[1]));
    }

    let a = read::<usize>();
    let mut cols = readv::<usize>();
    cols.insert(0, 0);
    cols.push(w);

    let b = read::<usize>();
    let mut rows = readv::<usize>();
    rows.insert(0, 0);
    rows.push(h);

    let mut cnt = std::collections::HashMap::new();
    for &(c, r) in pos.iter() {
        let i = partition_point(&rows, |&x| x < r);
        let j = partition_point(&cols, |&x| x < c);
        *cnt.entry((i, j)).or_insert(0) += 1;
        // println!("{} {} -> {} {}: {:?}", r, c, i, j, cnt);
    }

    let max = cnt.values().max().unwrap();
    let min = if cnt.len() == (a + 1) * (b + 1) {
        *cnt.values().min().unwrap()
    } else {
        0
    };

    println!("{} {}", min, max);
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
