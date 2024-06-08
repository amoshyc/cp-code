#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (l, r) = (inp[0], inp[1]);

    let mut idx = l;
    let mut ans = vec![];
    while idx < r {
        for i in (0..=60).rev() {
            let l = 1i64 << i;
            if idx + l <= r && idx % l == 0 {
                let j = idx / l;
                idx += l;
                let s = format!("{} {}", l * j, l * (j + 1));
                ans.push(s);
                break;
            }
        }
    }

    println!("{}", ans.len());
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
    read::<String>().chars().collect::<_>()
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
