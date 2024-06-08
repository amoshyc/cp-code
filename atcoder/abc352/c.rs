#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr_s = vec![];
    let mut arr_h = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        arr_s.push(inp[0]);
        arr_h.push(inp[1]);
    }

    let sum_s = arr_s.iter().sum::<i64>();

    let mut ans = 0;
    for i in 0..n {
        let val = sum_s - arr_s[i] + arr_h[i];
        ans = ans.max(val);
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
    read::<String>().chars().collect::<_>()
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
