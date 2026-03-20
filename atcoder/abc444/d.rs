#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut arr: [usize; n],
    }

    arr.sort();

    let mut res = vec![];
    let mut carry = 0;
    for digit in 0..=200_000 {
        let cnt = n - arr.partition_point(|x| *x <= digit); // number of items that is > digit
        let val = cnt + carry;
        if val == 0 {
            break;
        }

        res.push(val % 10);
        carry = val / 10;
    }
    while carry > 0 {
        res.push(carry % 10);
        carry /= 10;
    }

    res.reverse();
    println!("{}", join(&res, ""));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
