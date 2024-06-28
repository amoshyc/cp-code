#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (sx, sy) = (inp[0], inp[1]);
    let inp = readv::<i64>();
    let (tx, ty) = (inp[0], inp[1]);

    let to_left = |x: i64, y: i64| {
        if y % 2 == 0 {
            floor_div(x, 2) * 2
        } else {
            floor_div(x - 1, 2) * 2 + 1
        }
    };

    let sx = to_left(sx, sy);
    let tx = to_left(tx, ty);

    let dy = (ty - sy).abs();

    let mut ans = dy;

    let lb = sx - dy;
    let ub = sx + 1 + dy;
    if lb <= tx && tx <= ub {
        ans += 0;
    } else {
        let v1 = (tx - lb).abs() / 2;
        let v2 = (tx + 1 - ub).abs() / 2;
        ans += v1.min(v2);
    }

    println!("{}", ans);
}

fn floor_div(a: i64, b: i64) -> i64 {
    a.div_euclid(b)
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
