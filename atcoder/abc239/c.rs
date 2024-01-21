#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (x1, y1) = (inp[0], inp[1]);
    let (x2, y2) = (inp[2], inp[3]);
    for x in (x1 - 10)..=(x1 + 10) {
        for y in (y1 - 10)..=(y1 + 10) {
            let d1 = (x1 - x).pow(2) + (y1 - y).pow(2);
            let d2 = (x2 - x).pow(2) + (y2 - y).pow(2);
            if d1 == 5 && d2 == 5 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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