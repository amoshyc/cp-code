#![allow(unused)]


fn main() {
    let (n, m) = read2::<usize, usize>();
    let mut deg = vec![0; n];
    for _ in 0..m {
        let (u, v) = read2::<usize, usize>();
        let (u, v) = (u - 1, v - 1);
        deg[v] += 1;
    }

    let ans = (0..n).filter(|&i| deg[i] == 0).collect::<Vec<_>>();
    if ans.len() != 1 {
        println!("-1");
    } else {
        println!("{}", ans[0] + 1);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
