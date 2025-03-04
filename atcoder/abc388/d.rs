#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    let mut ans = vec![0; n];
    let mut diff = vec![0; n];
    let mut pref = 0;
    for i in 0..n {
        pref += diff[i];
        let x = arr[i] + pref;
        let r = (i as i64 + x).min(n as i64 - 1) as usize;
        if i + 1 < n {
            diff[i + 1] += 1;
        }
        if r + 1 < n {
            diff[r + 1] -= 1;
        }
        ans[i] = x - (r - i) as i64;
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
