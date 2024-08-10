#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (w, h, x, y) = (inp[0], inp[1], inp[2], inp[3]);

    let mut cnt = 0;
    if w % 2 == 0 && x == w / 2 {
        cnt += 1;
    }
    if h % 2 == 0 && y == h / 2 {
        cnt += 1;
    }

    if cnt == 2 {
        println!("{} 1", w * h / 2);
    } else {
        println!("{:.10} 0", ((w * h) as f64) / 2.0);
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
