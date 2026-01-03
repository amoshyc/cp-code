#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
    }

    let mut stack = vec![];
    for &x in &arr {
        let m = stack.len();
        if m >= 3 && stack[m - 3..] == [x, x, x] {
            stack.pop();
            stack.pop();
            stack.pop();
        } else {
            stack.push(x);
        }
    }

    println!("{}", stack.len());
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
