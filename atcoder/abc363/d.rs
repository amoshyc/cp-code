#![allow(unused)]

fn main() {
    let mut x = read::<i64>();
    if x == 1 {
        println!("0");
        return;
    }

    x -= 1;

    let f = |k: i64| 9 * 10i64.pow(((k + 1) / 2) as u32 - 1);
    let g = |x: i64, k: i64| 10i64.pow(((k + 1) / 2) as u32 - 1) + x;
    for k in (1..) {
        if f(k) < x {
            x -= f(k);
        } else {
            let pref: Vec<char> = g(x - 1, k).to_string().chars().collect();
            if k % 2 == 1 {
                let suff: Vec<_> = pref.iter().rev().skip(1).collect();
                println!("{}{}", join(&pref, ""), join(&suff, ""));
            } else {
                let suff: Vec<_> = pref.iter().rev().collect();
                println!("{}{}", join(&pref, ""), join(&suff, ""));
            }
            return;
        }
    }
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
