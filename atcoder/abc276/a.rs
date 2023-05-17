fn main() {
    let s = reads();
    let mut ans = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            ans = (i + 1) as i32;
        }
    }
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}
