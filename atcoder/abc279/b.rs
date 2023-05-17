fn main() {
    let s = read::<String>();
    let t = read::<String>();
    let ans = if s.contains(&t) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// fn readv<T: std::str::FromStr>() -> Vec<T> {
//     read::<String>()
//         .split_ascii_whitespace()
//         .map(|t| t.parse().ok().unwrap())
//         .collect()
// }

// fn reads() -> Vec<char> {
//     read::<String>().chars().collect::<Vec<char>>()
// }