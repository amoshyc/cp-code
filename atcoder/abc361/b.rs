#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (x1, y1, z1) = (inp[0], inp[1], inp[2]);
    let (x2, y2, z2) = (inp[3], inp[4], inp[5]);
    let inp = readv::<i64>();
    let (x3, y3, z3) = (inp[0], inp[1], inp[2]);
    let (x4, y4, z4) = (inp[3], inp[4], inp[5]);

    let dx = x2.min(x4) - x1.max(x3);
    let dy = y2.min(y4) - y1.max(y3);
    let dz = z2.min(z4) - z1.max(z3);
    if dx * dy * dz > 0 {
        println!("Yes");
    } else {
        println!("No");
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
