fn main() {
    let mut s = reads();
    let mut t = s.clone();
    let mut ans = vec![];
    ans.push(s.clone());
    for _ in 0..s.len() {
        s.rotate_left(1);
        t.rotate_right(1);
        ans.push(s.clone());
        ans.push(t.clone());
    }
    ans.sort();
    println!("{}", join(&ans[0], ""));
    println!("{}", join(&ans[ans.len() - 1], ""));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
