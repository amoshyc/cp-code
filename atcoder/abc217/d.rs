#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (l, q) = (inp[0], inp[1]);
    let mut cuts = std::collections::BTreeSet::new();
    cuts.insert(0);
    cuts.insert(l);
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            cuts.insert(inp[1]);
        } else {
            let x = inp[1];
            let a = cuts.range(..x).last().unwrap();
            let b = cuts.range(x..).next().unwrap();
            ans.push(b - a);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
