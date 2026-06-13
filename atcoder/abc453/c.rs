#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut arr: [isize; n],
    }

    for i in 0..n {
        arr[i] *= 2;
    }

    let mut max = 0;
    for m in 0..(1 << n) {
        let mut pos = 1;
        let mut cnt = 0;
        for i in 0..n {
            let offset = if (m >> i) & 1 == 1 { arr[i] } else { -arr[i] };
            let new_pos = pos + offset;

            if (new_pos < 0 && pos > 0) || (new_pos > 0 && pos < 0) {
                cnt += 1;
            }

            pos = new_pos;
        }

        max = max.max(cnt);
    }

    println!("{max}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
