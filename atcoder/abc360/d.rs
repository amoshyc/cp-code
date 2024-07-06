#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, t) = (inp[0] as usize, inp[1]);
    let s = reads();
    let x = readv::<i64>();

    let mut x0: Vec<_> = (0..n).filter(|i| s[*i] == '0').map(|i| x[i]).collect();
    let mut x1: Vec<_> = (0..n).filter(|i| s[*i] == '1').map(|i| x[i]).collect();

    x0.sort();
    x1.sort();

    let count = |arr: &Vec<i64>, lb: i64, ub: i64| -> i64 {
        let l = arr.partition_point(|x| *x < lb);
        let r = arr.partition_point(|x| *x <= ub);
        (r - l) as i64
    };

    let mut ans = 0;
    for i in 0..n {
        if s[i] == '0' {
            ans += count(&x1, x[i] - 2 * t, x[i]);
        } else {
            ans += count(&x0, x[i], x[i] + 2 * t);
        }
    }

    println!("{}", ans / 2);
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
