#![allow(unused)]

fn main() {
    let r = read::<i64>();
    let ys: Vec<i64> = (0..=r).collect();
    let mut ans = 0;
    for x in (-r)..=r {
        // (x +- 0.5)^2 + (y +- 0.5)^2 <= r^2
        // x^2 + 0.25 +- x + y^2 + 0.25 +- y <= r^2
        // x^2 + y^2 + 0.5 +- x +- y <= r^2
        // 2 x^2 + 2 y^2 + 1 +- 2x +- 2y <= 2 r^2

        let ymax = ys.partition_point(|y| {
            let mut ok = true;
            ok &= 2 * x * x + 2 * y * y + 1 + 2 * x + 2 * y <= 2 * r * r;
            ok &= 2 * x * x + 2 * y * y + 1 - 2 * x + 2 * y <= 2 * r * r;
            ok &= 2 * x * x + 2 * y * y + 1 + 2 * x - 2 * y <= 2 * r * r;
            ok &= 2 * x * x + 2 * y * y + 1 - 2 * x - 2 * y <= 2 * r * r;
            ok
        });

        if ymax > 0 {
            ans += 2 * ymax - 1;
        }
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
