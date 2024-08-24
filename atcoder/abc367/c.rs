#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let r = readv::<usize>();
    let mut arr = vec![0; n];
    dfs(0, &mut arr, &r, k);
}

fn dfs(i: usize, arr: &mut Vec<usize>, r: &Vec<usize>, k: usize) {
    let n = arr.len();
    if i == n {
        let s = arr.iter().sum::<usize>();
        if s % k == 0 {
            println!("{}", join(&arr, " "));
        }
        return;
    }

    for v in 1..=r[i] {
        arr[i] = v;
        dfs(i + 1, arr, r, k);
        arr[i] = 0;
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
    read::<String>().chars().collect()
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
