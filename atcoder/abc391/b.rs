#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut s = vec![];
    for _ in 0..n {
        s.push(reads());
    }
    let mut t = vec![];
    for _ in 0..m {
        t.push(reads());
    }

    for r in (0..).take_while(|r| r + m <= n) {
        for c in (0..).take_while(|c| c + m <= n) {
            let mut ok = true;
            for i in 0..m {
                for j in 0..m {
                    if t[i][j] != s[r + i][c + j] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("{} {}", r + 1, c + 1);
                return;
            }
        }
    }
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
