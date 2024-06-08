#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![vec!['.'; w]; h];
    let mut r = 0;
    let mut c = 0;
    let mut d = 0;
    let mut dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    for _ in 0..n {
        if arr[r][c] == '.' {
            arr[r][c] = '#';
            d = (d + 1) % 4;
        } else {
            arr[r][c] = '.';
            d = (d + 4 - 1) % 4;
        }
        let (dr, dc) = dirs[d];
        r = r.checked_add_signed(dr).unwrap_or(h - 1) % h;
        c = c.checked_add_signed(dc).unwrap_or(w - 1) % w;
    }

    for r in 0..h {
        println!("{}", join(&arr[r], ""));
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
