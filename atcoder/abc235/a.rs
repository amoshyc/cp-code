#![allow(unused)]

fn main() {
    let inp = read::<usize>();
    let (a, b, c) = (inp / 100, inp / 10 % 10, inp % 10);
    let abc = 100 * a + 10 * b + c;
    let bca = 100 * b + 10 * c + a;
    let cab = 100 * c + 10 * a + b;
    println!("{}", abc + bca + cab);
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
