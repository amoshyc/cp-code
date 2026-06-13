#![allow(unused)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let (n, m) = (s.len(), t.len());
    let ord = |c| c as usize - 'a' as usize;

    // For all the substring starting from l, find the number that does not have T as subseq.
    // We simply find the next char of T in S iteratively, starting from l.
    // If we find all the char of T and the last char is at position r, then all substring S[l..=(<r)] do not contain T as subseq.

    // We can find all the position of each alphabet in prior and use binary search.
    // Or we can use dp to find next position for each alphabet at each position.

    let mut next = vec![vec![n; n]; 26];
    for c in 'a'..='z' {
        for i in (0..n).rev() {
            if s[i] == c {
                next[ord(c)][i] = i;
            } else if i + 1 < n {
                next[ord(c)][i] = next[ord(c)][i + 1];
            }
        }
    }

    let mut ans = 0;
    for l in 0..n {
        // find all the char of t starting from l
        let mut r = next[ord(t[0])][l];
        for c in t.iter().skip(1) {
            if r + 1 < n {
                r = next[ord(*c)][r + 1];
            } else {
                // Unable find all the char
                r = n;
                break;
            }
        }

        // All valid substring:
        // S[l..l + 1], S[l..l + 2], ..., S[l..r]
        ans += (r - l) as i64;
    }

    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
