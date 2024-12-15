#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, d) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut pos = vec![];
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '.' {
                pos.push((r, c));
            }
        }
    }

    let dis = |p1: (usize, usize), p2: (usize, usize)| -> usize {
        let (r1, c1) = p1;
        let (r2, c2) = p2;
        r1.abs_diff(r2) + c1.abs_diff(c2)
    };

    let mut ans = 0;
    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            let mut cnt = 0;
            for r in 0..h {
                for c in 0..w {
                    if arr[r][c] == '.' {
                        if dis((r, c), pos[i]) <= d || dis((r, c), pos[j]) <= d {
                            cnt += 1;
                        }
                    }
                }
            }
            ans = ans.max(cnt);
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
