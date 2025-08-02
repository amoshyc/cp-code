#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, s) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut ok = true;
    if arr[0] > s {
        ok = false;
    }
    for i in 1..n {
        if arr[i] - arr[i - 1] > s {
            ok = false;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
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
