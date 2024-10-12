#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut ans = vec![vec![' '; n]; n];
    for r in 0..n {
        for c in 0..n {
            let cnt = (r + 1).min(c + 1).min(n - r).min(n - c) % 4;
            let mut nr = r;
            let mut nc = c;
            for _ in 0..cnt {
                (nr, nc) = (nc, n - 1 - nr);
            }
            ans[nr][nc] = arr[r][c];
        }
    }

    let mut out = vec![];
    for r in 0..n {
        out.push(join(&ans[r], ""));
    }
    println!("{}", join(&out, "\n"));
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
