#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![false; n]; n];
    for i in 0..m {
        let arr = readv::<usize>();
        for w in arr.windows(2) {
            let (a, b) = (w[0] - 1, w[1] - 1);
            adj[a][b] = true;
            adj[b][a] = true;
        }
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if !adj[i][j] {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
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
