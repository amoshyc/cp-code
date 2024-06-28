#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut h = readv::<i64>();
    h.push(0);

    let mut ans = vec![0; n + 1];
    let mut cum = 0;
    let mut stk = vec![];
    stk.push((i64::MAX, -1));

    for i in 0..=n {
        ans[i] = cum + 1;

        while stk.last().map_or(false, |&(x, p)| h[i] > x) {
            let (x, p) = stk[stk.len() - 1];
            let (y, q) = stk[stk.len() - 2];
            cum -= x * (p - q);
            stk.pop();
        }

        let (x, p) = stk.last().unwrap();
        cum += h[i] * (i as i64 - p);
        stk.push((h[i], i as i64));
    }

    println!("{}", join(&ans[1..], " "));
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
