#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        mut n: i64,
        k: i64,
    }

    let mut cnt = 0;
    for i in (0..) {
        cnt += n;
        n += 1;
        if cnt >= k {
            println!("{i}");
            break;
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
