#![allow(unused)]

fn main() {
    let t = read::<usize>();
    let mut out = vec![];
    for _ in 0..t {
        let inp = readv::<i64>();
        let (n, d, k) = (inp[0], inp[1], inp[2]);

        // linear congruence ax = b (mod m) has:
        // if b % m != 0 then no solution
        // if b % m == 0 then x = (a/g)^(-1)(b/g) (mod m/g)

        // r + xd = r (mod n)
        // xd = 0 (mod n)
        // x = 0 (mod n / g)
        // x = n / g
        let g = gcd(d, n);
        let jump = (k - 1) / (n / g) % n;
        let next = (k - 1) % (n / g) % n;
        let ans = (jump + next * (d % n)) % n;
        out.push(ans);
    }
    println!("{}", join(&out, "\n"));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
