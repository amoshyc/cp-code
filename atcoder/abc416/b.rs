#![allow(unused)]

fn main() {
    let s = reads();
    let n = s.len();
    let mut t = s.clone();

    let mut indices: Vec<usize> = (0..n).filter(|i| s[*i] == '#').collect();

    for w in indices.windows(2) {
        if w[1] > w[0] + 1 {
            t[w[0] + 1] = 'o';
        }
    }

    if indices.len() == 0 {
        t[0] = 'o';
    } else {
        if t[0] == '.' {
            t[0] = 'o';
        }
        if t[n - 1] == '.' {
            t[n - 1] = 'o';
        }
    }

    println!("{}", join(&t, ""));
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
