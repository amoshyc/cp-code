#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr_a = readv::<usize>();
    let arr_a = mapv(&arr_a, |&x| x - 1);
    let mut arr_w = readv::<usize>();

    let mut inv = vec![vec![]; n];
    for i in 0..n {
        inv[arr_a[i]].push(arr_w[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        if inv[i].len() > 1 {
            inv[i].sort();
            inv[i].reverse();
            ans += inv[i][1..].iter().sum::<usize>();
        }
    }
    println!("{}", ans);
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
