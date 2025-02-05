#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    let mut ans: i64 = 0;
    let m = 10i64.pow(9) + 7;

    for k in 0..60 {
        let mut cnt_0 = 0;
        let mut cnt_1 = 0;
        for i in 0..n {
            if (arr[i] >> k) & 1 == 1 {
                ans += (cnt_0 * ((1 << k) % m)) % m;
                cnt_1 += 1;
            } else {
                ans += (cnt_1 * ((1 << k) % m)) % m;
                cnt_0 += 1;
            }
            ans %= m;
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
