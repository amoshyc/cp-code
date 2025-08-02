#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (a, b, x) = (inp[0], inp[1], inp[2]);

    let ok = |n: i64| -> bool { a * n + b * (n.to_string().len() as i64) <= x };

    // 1 1 1 0 0 0
    //     ^
    let mut lb = 1;
    let mut ub = 1_000_000_000;
    if !ok(lb) {
        println!("0");
        return;
    }
    if ok(ub) {
        println!("{}", ub);
        return;
    }
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{}", lb);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
