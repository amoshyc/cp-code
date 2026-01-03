#![allow(unused)]

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let mut set = HashSet::new();
    set.insert(n);

    while n != 1 {
        let mut x = 0;
        let mut y = n;
        while y > 0 {
            x += (y % 10) * (y % 10);
            y /= 10;
        }
        n = x;

        if set.contains(&n) {
            println!("No");
            return;
        } else {
            set.insert(n);
        }
    }

    println!("Yes");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
