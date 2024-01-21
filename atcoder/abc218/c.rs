#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = vec![];
    let mut t = vec![];
    for r in 0..n {
        s.push(reads());
    }
    for r in 0..n {
        t.push(reads());
    }

    let norm = |arr: &Vec<Vec<char>>| {
        let mut dr = n;
        let mut dc = n;
        for r in (0..n).rev() {
            for c in (0..n).rev() {
                if arr[r][c] == '#' {
                    dr = dr.min(r);
                    dc = dc.min(c);
                }
            }
        }
        let mut res = arr.clone();
        for r in 0..n {
            for c in 0..n {
                if r + dr < n && c + dc < n {
                    res[r][c] = arr[r + dr][c + dc];
                } else {
                    res[r][c] = '.';
                }
            }
        }
        res
    };

    t = norm(&t);

    for i in 0..4 {
        if norm(&s) == t {
            println!("Yes");
            return;
        }
        s = r90cw(&s);
    }

    println!("No");
}

fn r90cw<T: Default + Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (n, m) = (arr.len(), arr[0].len());
    let mut res = vec![vec![T::default(); n]; m];
    for r in 0..n {
        for c in 0..m {
            res[r][c] = arr[c][n - 1 - r].clone();
        }
    }
    res
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
