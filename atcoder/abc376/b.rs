#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut pos_l = 0 as usize;
    let mut pos_r = 1 as usize;
    let mut ans = 0;

    for _ in 0..q {
        let ask = readv::<String>();
        let h = ask[0].chars().next().unwrap();
        let t = ask[1].parse::<usize>().unwrap() - 1;
        if h == 'L' {
            let (src, dst) = (pos_l.min(t), pos_l.max(t));
            // d1 = dst - src, d2 = n - d1
            if src <= pos_r && pos_r <= dst {
                ans += n - (dst - src);
            } else {
                ans += dst - src;
            }
            pos_l = t;
        } else {
            let (src, dst) = (pos_r.min(t), pos_r.max(t));
            // d1 = dst - src, d2 = n - d1
            if src <= pos_l && pos_l <= dst {
                ans += n - (dst - src);
            } else {
                ans += dst - src;
            }
            pos_r = t;
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
