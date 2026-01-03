#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }

    let mut r = 1;
    let mut cnt = 0;
    for i in 1..=n {
        if r >= i as i64 {
            r = r.max(i as i64 + h[i - 1] - 1);
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{cnt}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
