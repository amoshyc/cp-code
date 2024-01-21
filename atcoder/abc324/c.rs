#![allow(unused)]

fn main() {
    let inp = readv::<String>();
    let n = inp[0].parse::<usize>().unwrap();
    let t = inp[1].chars().collect::<Vec<_>>();

    let mut rt = t.clone();
    rt.reverse();

    let mut ans = vec![];
    for i in 0..n {
        let s = reads();
        let mut rs = s.clone();
        rs.reverse();

        // maximum common prefix
        let a = (0..s.len().min(t.len()))
            .take_while(|&i| s[i] == t[i])
            .count();
        // maximum common suffix
        let b = (0..s.len().min(t.len()))
            .take_while(|&i| rs[i] == rt[i])
            .count();

        let mut ok = false;
        ok |= s.len() == t.len() && a == t.len();
        ok |= s.len() == t.len() + 1 && a + b + 1 >= s.len();
        ok |= s.len() + 1 == t.len() && a + b + 1 >= t.len();
        ok |= s.len() == t.len() && a + b + 1 == s.len();
        if ok {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.len());
    println!("{}", join(&ans, " "));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
