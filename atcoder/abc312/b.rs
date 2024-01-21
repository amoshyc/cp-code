#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let t1 = vec!["###.", "###.", "###.", "...."];
    let t2 = vec!["....", ".###", ".###", ".###"];

    let t1 = mapv(&t1, |s| s.chars().collect::<Vec<char>>());
    let t2 = mapv(&t2, |s| s.chars().collect::<Vec<char>>());

    let mut ans = vec![];
    for i in (0..).take_while(|i| i + 9 <= n) {
        for j in (0..).take_while(|j| j + 9 <= m) {
            let mut ok = true;

            for a in 0..4 {
                for b in 0..4 {
                    if arr[i + a][j + b] != t1[a][b] {
                        ok = false;
                    }
                    if arr[i + 5 + a][j + 5 + b] != t2[a][b] {
                        ok = false;
                    }
                }
            }

            if ok {
                ans.push(format!("{} {}", i + 1, j + 1));
            }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
