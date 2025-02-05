#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut cnt = vec![1; n];
    let mut pos: Vec<usize> = (0..n).collect();
    let mut val = 0;

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let p = ask[1] - 1;
            let h = ask[2] - 1;
            let x = pos[p];
            cnt[x] -= 1;
            if cnt[x] == 1 {
                val -= 1;
            }
            cnt[h] += 1;
            if cnt[h] == 2 {
                val += 1;
            }
            pos[p] = h;
        } else {
            ans.push(val);
        }
    }
    println!("{}", join(&ans, "\n"));
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
