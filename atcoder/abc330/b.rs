#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, l, r) = (inp[0] as usize, inp[1], inp[2]);
    let arr = readv::<i64>();

    let mut ans = vec![];
    for i in 0..n {
        let mut cands = vec![l, r];
        if arr[i] >= l && arr[i] <= r {
            cands.push(arr[i]);
        }
        let val = cands.iter().min_by_key(|&y| (y - arr[i]).abs());
        ans.push(*val.unwrap());
    }

    println!("{}", join(&ans, " "));
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
