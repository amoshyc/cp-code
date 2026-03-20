#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    let mut res = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            d: usize,
            arr_a: [usize; n],
            arr_b: [usize; n],
        }

        let mut eggs = VecDeque::new();
        for i in 0..n {
            for _ in 0..arr_a[i] {
                eggs.push_back(i);
            }
            for _ in 0..arr_b[i] {
                eggs.pop_front();
            }
            while let Some(x) = eggs.front() {
                if i - x >= d {
                    eggs.pop_front();
                } else {
                    break;
                }
            }
        }

        res.push(eggs.len());
    }

    println!("{}", join(&res, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
