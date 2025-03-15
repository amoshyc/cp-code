#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![vec!['.'; n]; n];

    dfs(0, 0, n, &mut arr);
    for r in 0..n {
        println!("{}", join(&arr[r], ""));
    }
}

fn dfs(r: usize, c: usize, l: usize, arr: &mut Vec<Vec<char>>) {
    for i in 0..l {
        arr[r][c + i] = '#';
        arr[r + l - 1][c + i] = '#';
        arr[r + i][c] = '#';
        arr[r + i][c + l - 1] = '#';
    }

    if l >= 4 {
        dfs(r + 2, c + 2, l - 4, arr);
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
