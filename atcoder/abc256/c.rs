#![allow(unused)]

fn main() {
    let inp = readv::<i32>();
    let h = vec![inp[0], inp[1], inp[2]];
    let w = vec![inp[3], inp[4], inp[5]];

    let mut ans = 0;
    for a00 in 1..=30 {
        for a01 in 1..=30 {
            for a10 in 1..=30 {
                for a11 in 1..=30 {
                    let a02 = h[0] - a00 - a01;
                    let a12 = h[1] - a10 - a11;
                    let a20 = w[0] - a00 - a10;
                    let a21 = w[1] - a01 - a11;
                    let a22_1 = h[2] - a20 - a21;
                    let a22_2 = w[2] - a02 - a12;

                    if a22_1 != a22_2 {
                        continue;
                    }
                    if a02 <= 0 || a12 <= 0 || a20 <= 0 || a21 <= 0 || a22_1 <= 0 {
                        continue;
                    }
                    ans += 1;
                }
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
