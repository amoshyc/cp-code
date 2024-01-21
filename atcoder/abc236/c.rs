#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let s = readv::<String>();
    let t = readv::<String>();

    let mut set = std::collections::HashSet::new();
    for i in 0..m {
        set.insert(t[i].clone());
    }

    let mut ans = vec![];
    for i in 0..n {
        if set.contains(&s[i]) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }
    println!("{}", join(&ans, "\n"));
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
