#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut indices = vec![vec![]; n + 1];
    for (i, &a) in arr.iter().enumerate() {
        indices[a].push(i);
    }

    let mut ans = vec![];
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<usize>();
        let (a, b, x) = (inp[0] - 1, inp[1] - 1, inp[2]);
        let ub = partition_point(&indices[x], |&i| i <= b);
        let lb = partition_point(&indices[x], |&i| i < a);
        ans.push(ub - lb);
    }

    println!("{}", join(&ans, "\n"));
}


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