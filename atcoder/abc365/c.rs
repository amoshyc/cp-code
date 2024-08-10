#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, m) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let ok = |x: i64| -> bool { (0..n).map(|i| arr[i].min(x)).sum::<i64>() <= m };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 10i64.pow(10);
    if ok(ub) {
        println!("infinite");
        return;
    }
    while ub - lb > 1 {
        let mid = (lb + ub) / 2;
        if ok(mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    println!("{}", lb);
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
