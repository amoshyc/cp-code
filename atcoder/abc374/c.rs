#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    let mut ans = i64::MAX;
    for m in 0..(1 << n) {
        let mut cnt_a = 0;
        let mut cnt_b = 0;
        for i in 0..n {
            if (m >> i) & 1 != 0 {
                cnt_a += arr[i];
            } else {
                cnt_b += arr[i];
            }
        }
        ans = ans.min(cnt_a.max(cnt_b));
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