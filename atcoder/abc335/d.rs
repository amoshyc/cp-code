#![allow(unused)]

fn main() {
    let n = read::<usize>();

    let mut ans = vec![vec![0; n]; n];
    ans[n / 2][n / 2] = n * n;

    let mut r = 0;
    let mut c = 0;
    let mut d = 0;
    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for i in 1..(n * n) {
        ans[r][c] = i;

        for _ in 0..4 {
            let (dr, dc) = dirs[d];
            let nr = r.checked_add_signed(dr).unwrap_or(n);
            let nc = c.checked_add_signed(dc).unwrap_or(n);
            if nr < n && nc < n && ans[nr][nc] == 0 {
                r = nr;
                c = nc;
                break;
            } else {
                d = (d + 1) % 4;
            }
        }
    }

    for r in 0..n {
        let row = mapv(&ans[r], |&x| {
            if x == n * n {
                "T".to_string()
            } else {
                x.to_string()
            }
        });
        println!("{}", join(&row, " "));
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
