#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    while i + 1 < n {
        while j < n && arr[j] == arr[i] + (j - i) as i64 * (arr[i + 1] - arr[i]) {
            j += 1;
        }
        ans += (j - i) as i64;
        i += 1;
    }
    ans += 1;

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
