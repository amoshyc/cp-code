#![allow(unused)]

// [Problem]
// There is a rectangular cuboid cake with width A, depth B, and height C.
// You can perform the following operations on the cake:
//     1. Cut in a direction parallel to one of its faces.
//     2. However, the cake cannot be moved (if it is divided into multiple pieces, these pieces cannot be reshaped or cut separately).
// What is the minimum number of operations needed to turn all pieces into cubes?

// [Solution]
// The length of the resulting cubes is gcd(A, B, C).
// Making length L into pieces of length x needs L / x - 1 cuts.

fn main() {
    let inp = readv::<u64>();
    let (a, b, c) = (inp[0], inp[1], inp[2]);
    let g = gcd(a, gcd(b, c));
    let mut ans = 0;
    ans += a / g - 1;
    ans += b / g - 1;
    ans += c / g - 1;
    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
    read::<String>().chars().collect::<Vec<char>>()
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
