#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, a, b) = (inp[0], inp[1], inp[2]);
    let inp = readv::<i64>();
    let (p, q, r, s) = (inp[0], inp[1], inp[2], inp[3]);

    let mut ans = vec![];
    for i in p..=q {
        let mut row = vec![];
        for j in r..=s {
            if (i - a).abs() == (j - b).abs() {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        ans.push(join(&row, ""));
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
