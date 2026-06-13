#![allow(unused)]

use proconio::input;
fn main() {
    input! {
        x: usize,
    }

    for a in 1..=6 {
        for b in 1..=6 {
            for c in 1..=6 {
                if a + b + c == x {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
