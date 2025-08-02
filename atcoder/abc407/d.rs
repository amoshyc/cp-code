#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(readv::<u64>());
    }

    let mut mask = 0;
    let mut ans = 0;
    dfs(0, mask, &arr, &mut ans);

    println!("{}", ans);
}

fn dfs(i: usize, mask: u64, arr: &Vec<Vec<u64>>, ans: &mut u64) {
    let (h, w) = (arr.len(), arr[0].len());
    if i >= h * w {
        let mut xor = 0;
        for r in 0..h {
            for c in 0..w {
                if (mask >> (r * w + c)) & 1 == 0 {
                    xor ^= arr[r][c];
                }
            }
        }
        *ans = (*ans).max(xor);
        return;
    }

    let (r, c) = (i / w, i % w);

    // don't set
    dfs(i + 1, mask, arr, ans);

    // horizontal
    if c >= 1 && (mask >> (r * w + (c - 1))) & 1 == 0 {
        let mut new_mask = mask;
        new_mask |= 1 << (r * w + c - 1);
        new_mask |= 1 << (r * w + c);
        dfs(i + 1, new_mask, arr, ans);
    }

    // vertical
    if r >= 1 && (mask >> ((r - 1) * w + c)) & 1 == 0 {
        let mut new_mask = mask;
        new_mask |= 1 << ((r - 1) * w + c);
        new_mask |= 1 << (r * w + c);
        dfs(i + 1, new_mask, arr, ans);
    }
}

fn kth_bit(x: u64, k: usize) -> u64 {
    (x >> k) & 1
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
