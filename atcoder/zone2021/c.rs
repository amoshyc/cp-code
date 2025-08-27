#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(readv::<i64>());
    }

    let ok = |m: i64| -> bool {
        // min(max(A), max(B), max(C), ...) >= m
        // => max(A) >= m, max(B) >= m, max(C) >= m, ...
        // => Are there x, y, z that max(x.A, y.A, z.A) >= m, max(x.B, y.B, z.B) >= m, ...
        // => Are there x, y, z that
        //        ((x.A >= m) or (y.A >= m) or (z.A >= m)) and
        //        ((x.B >= m) or (y.B >= m) or (z.B >= m)) and
        //        ...
        // Note that each entry is 0 or 1, which can be stored in bits.
        // => Are there x, y, z that bitmask(x) | bitmask(y) | bitmask(z) == 2^5 - 1
        // Note that there are only 2^5 kinds of bitmask.
        // => Store all the bitmasks in set in prior.

        let mut set = HashSet::new();
        for i in 0..n {
            let mut mask = 0;
            for j in 0..5 {
                if arr[i][j] >= m {
                    mask |= 1 << j;
                }
            }
            set.insert(mask);
        }
        for &mask_x in &set {
            for &mask_y in &set {
                for &mask_z in &set {
                    if mask_x | mask_y | mask_z == (1 << 5) - 1 {
                        return true;
                    }
                }
            }
        }
        false
    };

    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 10i64.pow(9) + 1;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{lb}");
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
