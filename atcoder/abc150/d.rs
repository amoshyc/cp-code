#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (n, m) = (inp[0] as usize, inp[1]);
    let arr = readv::<u64>();

    // x = a[0] * (p[0] + 0.5)
    // x = a[1] * (p[1] + 0.5)
    // ...
    // =>
    // x = (a[0] / 2) * (2 * p[0] + 1)
    // x = (a[1] / 2) * (2 * p[1] + 1)
    // ...
    // => x is lcm(a[i] / 2) and (x / (a[i] / 2)) is odd

    let x = arr.iter().fold(1, |acc, x| lcm(acc, *x / 2));
    if x > m {
        println!("0");
        return;
    }
    for a in arr {
        if (x / (a / 2)) % 2 == 0 {
            println!("0");
            return;
        }
    }
    // number of multiples of x that is odd in 1..=M
    // = (number of multiples of x in 1..=M) - (number of multiples of 2x in 1..=M)
    let ans = m / x - m / (2 * x);
    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)).saturating_mul(b)
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
