#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        q: usize,
    }

    let mut s = vec![];
    let mut pref = vec![0; q];
    let mut ans = vec![];

    let inf = 10i64.pow(15);

    for _ in 0..q {
        input! { ask: usize }
        if ask == 1 {
            input! { c: char }
            s.push(c);
            let i = s.len() - 1;
            if c == '(' {
                if s.len() == 1 {
                    pref[i] = 1;
                } else {
                    pref[i] = pref[i - 1] + 1;
                }
            } else if c == ')' {
                if i >= 1 && pref[i - 1] >= 1 {
                    pref[i] = pref[i - 1] - 1;
                } else {
                    pref[i] = inf;
                }
            }
        } else {
            s.pop();
        }

        if s.is_empty() || pref[s.len() - 1] == 0 {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
