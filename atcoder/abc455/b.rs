#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        arr: [Chars; h],
    }

    let mut ans = 0;
    for r0 in 0..h {
        for c0 in 0..w {
            for r1 in r0..h {
                for c1 in c0..w {
                    let mut ok = true;
                    for r in r0..=r1 {
                        for c in c0..=c1 {
                            ok &= arr[r][c] == arr[r0 + r1 - r][c0 + c1 - c];
                        }
                    }

                    if ok {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
