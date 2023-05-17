fn main() {
    let x = read::<i32>();
    if x % 100 == 0 {
        println!("100");
    } else {
        println!("{}", (x + 99) / 100 * 100 - x);
    }
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