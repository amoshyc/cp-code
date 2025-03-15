#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut scores = vec![vec![0; 2 * n]; 2 * n];
    for i in 0..(2 * n - 1) {
        let arr = readv::<usize>();
        for j in 0..arr.len() {
            scores[i][i + 1 + j] = arr[j];
            scores[i + 1 + j][i] = arr[j];
        }
    }

    let mut ans = 0;
    let mut used = vec![false; 2 * n];
    dfs(0, &mut used, &mut ans, &scores);
    println!("{}", ans);
}

fn dfs(val: usize, used: &mut Vec<bool>, ans: &mut usize, scores: &Vec<Vec<usize>>) {
    let avail: Vec<usize> = (0..used.len()).filter(|&i| !used[i]).collect();

    if avail.len() == 0 {
        *ans = (*ans).max(val);
        return;
    }

    let x = avail[0];
    for &y in avail.iter().skip(1) {
        used[x] = true;
        used[y] = true;
        dfs(val ^ scores[x][y], used, ans, scores);
        used[y] = false;
        used[x] = false;
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
