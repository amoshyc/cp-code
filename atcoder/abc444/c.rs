#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut arr: [i64; n],
    }

    // l, l, l, ...

    // a, b, ..., l - b, l - a, l, l, l
    // a, b, ..., l - b, l - a

    // Case 1: max(A) is L, others (maybe empty) should agree.
    // Case 2: everyone agrees.

    let agree = |data: &Vec<i64>| -> bool {
        let n = data.len();
        if n % 2 == 1 {
            return false;
        }
        (0..n / 2).all(|i| data[i] + data[n - 1 - i] == data[0] + data[n - 1])
    };

    let mut ans = vec![];

    arr.sort();

    let max = arr[n - 1];
    let others = arr
        .iter()
        .filter(|&&x| x != max)
        .cloned()
        .collect::<Vec<_>>();
    let m = others.len();
    if agree(&others) {
        if m == 0 || others[0] + others[m - 1] == max {
            ans.push(max);
        }
    }

    if agree(&arr) {
        ans.push(arr[0] + arr[n - 1]);
    }

    ans.sort();
    println!("{}", join(&ans, " "));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
