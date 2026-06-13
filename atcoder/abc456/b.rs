#![allow(unused)]

use proconio::input;
fn main() {
    input! {
        arr: [[usize; 6]; 3]
    }

    let mut cnt = 0;
    for &a in &arr[0] {
        for &b in &arr[1] {
            for &c in &arr[2] {
                let mut x = vec![a, b, c];
                x.sort();
                if x == [4, 5, 6] {
                    cnt += 1;
                }
            }
        }
    }

    println!("{:.7}", (cnt as f64) / (6.0 * 6.0 * 6.0));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
