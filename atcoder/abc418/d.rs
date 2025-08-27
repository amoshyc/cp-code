#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let f = (0..n)
        .map(|i| if s[i] == '0' { 1 } else { 0 })
        .collect::<Vec<i64>>();

    let mut pref = vec![f[0]; n];
    for i in 1..n {
        pref[i] = pref[i - 1] + f[i];
    }

    // s[i..=j] is good if number of 0 in s[i..=j] is even
    // = (pref[j] - pref[i - 1] is even) or (pref[j] is even)

    let mut ans = 0;
    let mut cnt = vec![0 as i64; 2];
    for j in 0..n {
        let m = (pref[j] % 2) as usize;

        ans += cnt[m];
        if m == 0 {
            ans += 1;
        }

        cnt[m] += 1;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
