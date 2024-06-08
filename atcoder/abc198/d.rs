#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn main() {
    let s1 = reads();
    let s2 = reads();
    let s3 = reads();

    let mut chars = HashSet::new();
    chars.extend(s1.clone());
    chars.extend(s2.clone());
    chars.extend(s3.clone());

    if chars.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let mut mapping = HashMap::new();
    for (i, &c) in chars.iter().enumerate() {
        mapping.insert(c, i);
    }
    let s1 = mapv(&s1, |c| mapping[c]);
    let s2 = mapv(&s2, |c| mapping[c]);
    let s3 = mapv(&s3, |c| mapping[c]);

    let mut p: Vec<i64> = (0..=9).collect();
    loop {
        if p[s1[0]] != 0 && p[s2[0]] != 0 && p[s3[0]] != 0 {
            let mut n1 = 0;
            for c in s1.iter() {
                n1 = n1 * 10 + p[*c];
            }
            let mut n2 = 0;
            for c in s2.iter() {
                n2 = n2 * 10 + p[*c];
            }
            let mut n3 = 0;
            for c in s3.iter() {
                n3 = n3 * 10 + p[*c];
            }

            if n1 + n2 == n3 {
                println!("{}\n{}\n{}", n1, n2, n3);
                return;
            }
        }

        if next_permutation(&mut p).is_none() {
            break;
        }
    }

    println!("UNSOLVABLE");
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
