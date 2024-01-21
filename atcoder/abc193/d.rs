#![allow(unused)]

fn main() {
    let k = read::<usize>();
    let mut cnt = vec![k; 10];
    let mut a: Vec<usize> = reads()
        .iter()
        .map(|&c| match c {
            '0'..='9' => c.to_digit(10).unwrap() as usize,
            _ => 0,
        })
        .collect();
    let mut b: Vec<usize> = reads()
        .iter()
        .map(|&c| match c {
            '0'..='9' => c.to_digit(10).unwrap() as usize,
            _ => 0,
        })
        .collect();

    for i in 0..4 {
        cnt[a[i]] -= 1;
        cnt[b[i]] -= 1;
    }

    let mut win = 0;
    let mut total = 0;
    for u in 1..=9 {
        for v in 1..=9 {
            a[4] = u;
            b[4] = v;
            if score(&a) > score(&b) {
                if u != v {
                    win += cnt[u] * cnt[v];
                } else if cnt[u] >= 2 {
                    win += cnt[u] * (cnt[u] - 1);
                }
            }
        }
    }

    let total = (9 * k - 8) * (9 * k - 9);
    println!("{:.6}", (win as f64) / (total as f64));
}

fn score(arr: &Vec<usize>) -> i64 {
    let mut cnt = vec![0; 10];
    for &x in arr.iter() {
        cnt[x] += 1;
    }
    let mut s = 0;
    for i in 1..=9 {
        s += (i as i64) * 10i64.pow(cnt[i] as u32);
    }
    s
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
