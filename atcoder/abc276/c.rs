#![allow(unused)]

fn main() {
    let _ = read::<usize>();
    let mut arr = readv::<usize>();
    let res = prev_permutation(&arr).unwrap();
    println!("{}", join(&res, " "));
}

fn next_permutation<T: Ord + Clone>(arr: &Vec<T>) -> Option<Vec<T>> {
    let mut res = (*arr).clone();
    let k = res.windows(2).rposition(|w| w[0] < w[1])?;
    let j = res.iter().rposition(|a| *a > arr[k]).unwrap();
    res.swap(k, j);
    res[(k + 1)..].reverse();
    Some(res)
}

fn prev_permutation<T: Ord + Clone>(arr: &Vec<T>) -> Option<Vec<T>> {
    let mut res = (*arr).clone();
    let k = res.windows(2).rposition(|w| w[0] > w[1])?;
    let j = res.iter().rposition(|a| *a < arr[k]).unwrap();
    res.swap(k, j);
    res[(k + 1)..].reverse();
    Some(res)
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
