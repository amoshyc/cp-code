#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let m = 10i64.pow(8);
    let mut arr = readv::<i64>();

    let mut ans = 0;
    let mut pref = 0;
    
    arr.sort();
    for i in 0..n {
        // cnt = number of element arr[j] (0 <= j < i) that arr[i] + arr[j] >= 10^8
        let add = arr[i] * (i as i64) + pref;
        let cnt = i - arr[..i].partition_point(|&x| x < m - arr[i]);
        let sub = (cnt as i64) * m;

        ans += add - sub;

        pref += arr[i];
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
