#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let inp = readv::<usize>();
    let mut sr = inp[0] - 1;
    let mut sc = inp[1] - 1;

    let mut arr = vec![];
    for r in 0..h {
        arr.push(reads());
    }

    let s = reads();
    for i in 0..s.len() {
        let (mut dr, mut dc) = (0, 0);
        match s[i] {
            'U' => dr -= 1,
            'D' => dr += 1,
            'L' => dc -= 1,
            'R' => dc += 1,
            _ => panic!(),
        }
        let nr = sr.checked_add_signed(dr).unwrap_or(h);
        let nc = sc.checked_add_signed(dc).unwrap_or(w);
        if nr < h && nc < w && arr[nr][nc] != '#' {
            (sr, sc) = (nr, nc);
        }
    }
    println!("{} {}", sr + 1, sc + 1);
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
