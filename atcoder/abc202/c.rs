#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr_a = readv::<usize>();
    let arr_b = readv::<usize>();
    let arr_c = readv::<usize>();

    let mut cnt = vec![0; n + 1];
    for a in arr_a {
        cnt[a] += 1;
    }

    let mut ans = 0;
    for j in 0..n {
        let val = arr_b[arr_c[j] - 1];
        ans += cnt[val] as i64;
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
