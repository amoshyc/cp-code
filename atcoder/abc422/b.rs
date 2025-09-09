#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ok = true;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '.' {
                continue;
            }
            let mut cnt = 0;
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w {
                    if s[nr][nc] == '#' {
                        cnt += 1;
                    }
                }
            }
            ok &= cnt == 2 || cnt == 4;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
