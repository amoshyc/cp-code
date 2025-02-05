#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let v = readv::<i64>();
    let c = readv::<i64>();

    let mut ans = 0;
    for m in 0..(1 << n) {
        let mut val = 0;
        for i in 0..n {
            if (m >> i) & 1 == 1 {
                val += v[i];
                val -= c[i];
            }
        }
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
