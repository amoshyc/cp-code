#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut ans = 0;
    for r in 0..(h - 1) {
        for c in 0..(w - 1) {
            let mut cnt = 0;
            if arr[r][c] == '.' {
                cnt += 1;
            }
            if arr[r + 1][c] == '.' {
                cnt += 1;
            }
            if arr[r][c + 1] == '.' {
                cnt += 1;
            }
            if arr[r + 1][c + 1] == '.' {
                cnt += 1;
            }
            if cnt == 1 || cnt == 3 {
                ans += 1;
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
