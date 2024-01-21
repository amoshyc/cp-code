#![allow(unused)]


fn main() {
    let inp = readv::<i64>();
    let (n, w) = (inp[0] as usize, inp[1]);
    let mut items = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        items.push((inp[0], inp[1]))
    }
    items.sort();
    items.reverse();

    let mut c = w;
    let mut ans = 0;
    for (a, b) in items {
        let x = b.min(c);
        ans += x * a;
        c -= x;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
