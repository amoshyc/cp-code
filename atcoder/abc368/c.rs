#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut h = readv::<i64>();
    let mut t = 1;
    for i in 0..n {
        let q = h[i] / 5;
        t += q * 3;
        h[i] -= q * 5;
        while h[i] > 0 {
            if t % 3 == 0 {
                h[i] -= 3;
            } else {
                h[i] -= 1;
            }
            t += 1;
        }
    }
    println!("{}", t - 1);
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
