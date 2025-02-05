#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut checks = vec![vec![]; n];
    for i in 0..n {
        let m = read::<usize>();
        for _ in 0..m {
            let inp = readv::<usize>();
            let (x, y) = (inp[0], inp[1]);
            checks[i].push((x - 1, y));
        }
    }

    let mut ans = 0;
    for mask in 0..(1 << n) {
        let is_honest: Vec<bool> = (0..n).map(|i| (mask >> i) & 1 > 0).collect();

        let mut ok = true;
        for i in 0..n {
            if is_honest[i] {
                for &(x, y) in checks[i].iter() {
                    if y == 1 {
                        ok &= is_honest[x];
                    } else {
                        ok &= is_honest[x] == false;
                    }
                }
            }
        }
        if ok {
            ans = ans.max(is_honest.iter().filter(|x| **x).count());
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
