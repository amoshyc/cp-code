#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (h, w) = (inp[0], inp[1]);

    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    let ans = match (h % 2, w % 2) {
        (0, 0) => h * w / 2,
        (0, 1) => h * w / 2,
        (1, 0) => h * w / 2,
        (1, 1) => h * w / 2 + 1,
        _ => panic!(),
    };

    println!("{}", ans);
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
