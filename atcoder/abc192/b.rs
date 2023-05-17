fn main() {
    let s = reads();
    let ok1 = s[0..].iter().step_by(2).all(|&c| c.is_lowercase());
    let ok2 = s[1..].iter().step_by(2).all(|&c| c.is_uppercase());
    println!("{}", if ok1 && ok2 { "Yes" } else { "No" });
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}
