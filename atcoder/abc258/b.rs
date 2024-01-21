#![allow(unused)]

fn main() {
    let n = read::<i32>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(
            reads()
                .iter()
                .map(|&c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>(),
        );
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..n {
            for &(dr, dc) in [(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)].iter() {
                let mut val = 0;
                for i in 0..n {
                    let nr = ((r + i * dr).rem_euclid(n)) as usize;
                    let nc = ((c + i * dc).rem_euclid(n)) as usize;
                    val = val * 10 + arr[nr][nc];
                }
                ans = ans.max(val);
            }
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
