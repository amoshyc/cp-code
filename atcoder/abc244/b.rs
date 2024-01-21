#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let t = reads();

    let mut dx = 1;
    let mut dy = 0;
    let mut x = 0;
    let mut y = 0;
    let dirs = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    for &t in t.iter() {
        if t == 'S' {
            x += dx;
            y += dy;
        } else {
            let p = dirs.iter().position(|&x| x == (dx, dy)).unwrap();
            let (nx, ny) = dirs[(p + 1) % 4];
            dx = nx;
            dy = ny;
        }
    }

    println!("{} {}", x, y);
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