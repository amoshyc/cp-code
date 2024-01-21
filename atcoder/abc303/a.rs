#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let s = reads();
    let t = reads();

    for i in 0..n {
        if s[i] != t[i] {
            if !vec![('o', '0'), ('0', 'o'), ('1', 'l'), ('l', '1')].contains(&(s[i], t[i])) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
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
