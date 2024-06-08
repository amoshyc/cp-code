#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);
    let t = reads();
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let ok = |mut r: i32, mut c: i32| -> bool {
        for i in 0..n {
            match t[i] {
                'L' => c -= 1,
                'R' => c += 1,
                'U' => r -= 1,
                'D' => r += 1,
                _ => panic!(),
            }
            if r < 0 || r >= h as i32 || c < 0 || c >= w as i32 {
                return false;
            }
            if arr[r as usize][c as usize] == '#' {
                return false;
            }
        }
        true
    };

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '.' && ok(r as i32, c as i32) {
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
