#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);

    let mut arr = vec![];
    for _ in 0..h {
        arr.push(reads());
    }

    let mut max_r = 0;
    let mut max_c = 0;
    let mut min_r = h;
    let mut min_c = w;
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '#' {
                max_r = max_r.max(r);
                min_r = min_r.min(r);
                max_c = max_c.max(c);
                min_c = min_c.min(c);
            }
        }
    }

    if min_r > max_r || min_c > max_c {
        println!("No");
        return;
    }

    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if arr[r][c] == '.' {
                println!("No");
                return;
            }
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
