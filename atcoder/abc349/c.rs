#![allow(unused)]

fn main() {
    let s = reads();
    let t = reads();
    let n = s.len();

    let mut patterns = vec![t.clone()];
    if t[2] == 'X' {
        patterns.push(t[..2].to_vec());
    }

    for p in patterns {
        let mut idx = 0;
        let mut ok = true;
        for c in p {
            if let Some(i) = (idx..n).find(|i| s[*i].to_ascii_uppercase() == c) {
                idx = i + 1;
            } else {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
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
