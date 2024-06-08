#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    // ABC 254D
    // f(x) = largest divisor of x that is square number
    // (a * b) is square number if and only if a / f(a) = b / f(b)

    let m = *arr.iter().max().unwrap();
    let mut f = vec![0; m + 1];
    for d in (1..).take_while(|&d| d * d <= m) {
        for x in ((d * d)..=m).step_by(d * d) {
            f[x] = d * d;
        }
    }

    let mut ans = 0;

    let mut cnt = std::collections::HashMap::new();
    for &a in arr.iter() {
        if a != 0 {
            *cnt.entry(a / f[a]).or_insert(0) += 1;
        }
    }
    for &v in cnt.values() {
        if v >= 1 {
            ans += v * (v - 1) / 2;
        }
    }

    let cnt_zero = arr.iter().filter(|&a| *a == 0).count();
    ans += cnt_zero * (n - cnt_zero);
    if cnt_zero >= 1 {
        ans += cnt_zero * (cnt_zero - 1) / 2;
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
    read::<String>().chars().collect::<Vec<char>>()
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
