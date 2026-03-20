#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        tc: usize,
    }

    let mut ans = vec![i64::MAX; tc];
    for t in 0..tc {
        input! {
            n: usize,
            w: usize,
            c: [i64; n],
        }

        let mut sum = vec![0; 2 * w];
        for i in 0..n {
            sum[i % (2 * w)] += c[i];
        }

        sum.extend(sum.clone());
        let pref = build(&sum);
        
        for i in 0..(2 * w) {
            ans[t] = ans[t].min(query(&pref, i, i + w));
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn build<T>(arr: &[T]) -> Vec<T>
where
    T: Default + Copy + std::ops::Add<Output = T>,
{
    let mut pref = vec![T::default(); arr.len()];
    pref[0] = arr[0];
    for i in 1..arr.len() {
        pref[i] = pref[i - 1] + arr[i];
    }
    pref
}

// i..j
fn query<T>(pref: &[T], i: usize, j: usize) -> T
where
    T: Default + Copy + std::ops::Sub<Output = T>,
{
    if i == j {
        T::default()
    } else if i == 0 {
        pref[j - 1]
    } else {
        pref[j - 1] - pref[i - 1]
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
