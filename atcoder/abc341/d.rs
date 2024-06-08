#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);

    let g = gcd(n, m);
    let lcm = n / g * m;

    // f(x, l, r)
    //  = number of multiple of x in a range [l, r]
    //  = max(0, floor(r / x) - ceil(l / x) + 1)
    let f = |x: i64, l: i64, r: i64| -> i64 { (r / x - (l + x - 1) / x + 1).max(0) };

    // K-th smallest (1-based) in a ordered sequence
    // <-> k element smaller than or equal to x
    // -> n(element <= x) >= k
    // <-> f(n, 1, x) + f(m, 1, x) - 2 * f(lcm, 1, x) >= k
    // 0 0 0 1 1 1
    let ok = |x: i64| -> bool { f(n, 1, x) + f(m, 1, x) - 2 * f(lcm, 1, x) >= k };

    let mut lb = 0;
    let mut ub = 10i64.pow(18) + 1;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }

    println!("{}", ub);
}

fn gcd(a: i64, b: i64) -> i64 {
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
