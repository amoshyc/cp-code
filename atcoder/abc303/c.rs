#![allow(unused)]

fn main() {
    let inp = readv::<i32>();
    let s = reads();
    let (n, m, h, k) = (inp[0], inp[1], inp[2], inp[3]);

    let mut rec = std::collections::HashSet::new();
    for _ in 0..m {
        let pos = readv::<i32>();
        rec.insert((pos[0], pos[1]));
    }

    let mut x = 0;
    let mut y = 0;
    let mut h = h;
    for &c in s.iter() {
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }

        if c == 'R' {
            x += 1;
        }
        if c == 'L' {
            x -= 1;
        }
        if c == 'D' {
            y -= 1;
        }
        if c == 'U' {
            y += 1;
        }

        if h < k && rec.contains(&(x, y)) {
            rec.remove(&(x, y));
            h = k;
        }
    }

    println!("Yes");
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
