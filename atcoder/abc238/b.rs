#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut pivots = vec![];
    let mut cumsum = 0;
    pivots.push(cumsum);
    for &x in arr.iter() {
        cumsum += x;
        cumsum %= 360;
        pivots.push(cumsum);
    }

    pivots.sort();
    pivots.dedup();
    let ans1 = pivots.windows(2).map(|w| w[1] - w[0]).max().unwrap();
    let ans2 = 360 - pivots[pivots.len() - 1];
    println!("{}", std::cmp::max(ans1, ans2));
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
