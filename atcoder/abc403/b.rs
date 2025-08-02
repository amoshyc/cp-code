#![allow(unused)]

fn main() {
    let t = reads();
    let u = reads();

    let n = t.len();
    let m = u.len();

    let check = |arr: Vec<char>| -> bool {
        for i in 0..m {
            if arr[i] != u[i] && arr[i] != '?' {
                return false;
            }
        }
        true
    };

    for i in m..=n {
        if check(t[i - m..i].to_vec()) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
