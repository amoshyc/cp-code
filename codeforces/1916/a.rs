#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (n, k) = (inp[0], inp[1]);
        let b = readv::<usize>();

        let mut p = 2023;
        let mut ok = true;
        for x in b.iter() {
            if p % x != 0 {
                ok = false;
                break;
            } else {
                p /= x;
            }
        }
        ok &= p > 0;

        if ok {
            println!("Yes");
            let mut res = vec![1; k];
            res[0] = p;
            println!("{}", join(&res, " "));
        } else {
            println!("No");
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
