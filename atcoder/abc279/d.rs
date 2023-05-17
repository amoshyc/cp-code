fn main() {
    // min ( b * n + a * (1 + n) ^ (-1/2) )
    // b + a * (-1/2) * (1 + n) ^ (-3/2) = 0
    // a/2 * (1 + n) ^ (-3/2) = b
    // (1 + n) ^ (-3/2) = 2b/a
    // 1 + n = (2b/a) ^ (-2/3)
    // n = (2 * b / a) ^ (-2/3) - 1
    let inp = readv::<f64>();
    let (a, b) = (inp[0], inp[1]);
    let n = ((2.0 * b) / a).powf(-2.0/3.0) - 1.0;
    let n1 = std::cmp::max(0, n.ceil() as i64);
    let n2 = std::cmp::max(0, n.floor() as i64);
    // println!("{} {}", n1, n2);
    let ans1 = (b * n1 as f64) + a * ((1 + n1) as f64).powf(-1.0/2.0);
    let ans2 = (b * n2 as f64) + a * ((1 + n2) as f64).powf(-1.0/2.0);
    println!("{:.7}", ans1.min(ans2));
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

// fn reads() -> Vec<char> {
//     read::<String>().chars().collect::<Vec<char>>()
// }