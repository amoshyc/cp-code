#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut ans = 0;
    let mut is_login = false;
    for _ in 0..n {
        let s = join(&reads(), "");
        match s.as_str() {
            "login" => is_login = true,
            "logout" => is_login = false,
            "private" => {
                if !is_login {
                    ans += 1
                }
            }
            _ => (),
        }
    }
    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
