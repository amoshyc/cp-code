#![allow(unused)]


fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut pos = vec![];
    for r in 0..n {
        let inp = reads();
        for c in 0..m {
            if inp[c] == 'o' {
                pos.push((r, c));
            }
        }
    }

    let mut ans = 1_000_000_000;
    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            let (r1, c1) = pos[i];
            let (r2, c2) = pos[j];
            let dr = (r1 as i32 - r2 as i32).abs();
            let dc = (c1 as i32 - c2 as i32).abs();
            ans = ans.min(dr + dc);
        }
    }

    println!("{}", ans);
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
