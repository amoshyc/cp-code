#![allow(unused)]

fn main() {
    let inp = readv::<String>();
    let s: Vec<char> = inp[0].chars().collect();

    for w in 1..s.len() {
        let mut tokens = vec![];
        for i in (0..s.len()).step_by(w) {
            let j = s.len().min(i + w);
            tokens.push(s[i..j].to_vec());
        }
        for c in 1..=w {
            let res: Vec<_> = tokens
                .iter()
                .filter(|x| x.len() >= c)
                .map(|x| x[c - 1])
                .collect();
            if join(&res, "") == inp[1] {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
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
