#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (a, b, n) = (inp[0], inp[1], inp[2]);

    // Let x = kB + r, we have following constraints:
    // 1) x is integer in range [0, N]
    // 2) k is integer in range [0, inf]
    // 3) r is integer in range [0, B - 1]
    // To maximize floor(Ax/B) - A * floor(x/B)
    // = maximize floor((A k B + Ar) / B) - A * floor((kB + r) / B)
    // = maximize floor(AK + Ar/B) - A * floor(k + r/B)
    // = maximize AK + floor(Ar/B) - A ( k + floor(r/B) )
    // = maximize floor(Ar / B)
    // which is a monotonically increasing function given A > 0, B > 0
    // Therefore the maximum occurs when r is maximize under constraints.

    let r = (b - 1).min(n);
    let ans = a * r / b;
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
