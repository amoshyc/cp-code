// use itertools::*;
use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: i32,
        a: [i64; n],
        q: i32,
    }

    let mut reset = -1;
    let mut gain = HashMap::new();
    for _ in 0..q {
        input! {cmd: i32}

        if cmd == 1 {
            input! {x : i64}
            gain.clear();
            reset = x;
        }
        else if cmd == 2 {
            input! {mut i: i32, x : i64}
            i -= 1;
            *gain.entry(i).or_insert(0) += x;
        }
        else {
            input! {mut i : i32}
            i -= 1;
            if reset == -1 {
                println!("{}", a[i as usize] + gain.get(&i).unwrap_or(&0));
            } else {
                println!("{}", reset + gain.get(&i).unwrap_or(&0));
            }
        }
    }
}
