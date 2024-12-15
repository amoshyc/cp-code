#![allow(unused)]

fn main() {
    let inp = readv::<String>();
    let n: usize = inp[0].parse().unwrap();
    let c1 = inp[1].chars().next().unwrap();
    let c2 = inp[2].chars().next().unwrap();

    let s = reads();
    let t: Vec<char> = s.iter().map(|&c| if c != c1 { c2 } else { c }).collect();
    println!("{}", join(&t, ""));
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
