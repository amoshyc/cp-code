#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let l = 3usize.pow(n as u32);
    let mut arr = vec![vec!['#'; l]; l];
    dfs(0, 0, n, &mut arr);

    for r in 0..arr.len() {
        println!("{}", join(&arr[r], ""));
    }
}

fn dfs(r1: usize, c1: usize, k: usize, arr: &mut Vec<Vec<char>>) {
    if k == 0 {
        return;
    }

    let l = 3usize.pow(k as u32) / 3;

    dfs(r1, c1, k - 1, arr);
    dfs(r1, c1 + l, k - 1, arr);
    dfs(r1, c1 + l + l, k - 1, arr);

    dfs(r1 + l, c1, k - 1, arr);
    for r in (r1 + l)..(r1 + l + l) {
        for c in (c1 + l)..(c1 + l + l) {
            arr[r][c] = '.';
        }
    }
    dfs(r1 + l, c1 + l + l, k - 1, arr);

    dfs(r1 + l + l, c1, k - 1, arr);
    dfs(r1 + l + l, c1 + l, k - 1, arr);
    dfs(r1 + l + l, c1 + l + l, k - 1, arr);
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
