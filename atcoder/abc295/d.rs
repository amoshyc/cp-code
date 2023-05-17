#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let h: Vec<u64> = vec![
        6068390343136051,
        9543240859151955,
        3897401881081837,
        2318393960333402,
        7016574820138116,
        1831374890149697,
        9785307505223662,
        3543710416817501,
        8942711339746222,
        1235305222514187,
    ];
    let s = reads();
    let arr = s
        .iter()
        .map(|&c| h[c.to_digit(10).unwrap() as usize])
        .collect::<Vec<u64>>();

    let mut ans = 0_u64;
    let mut p = vec![0];
    let mut cnt = HashMap::new();
    cnt.insert(0, 1);
    for r in 0..s.len() {
        let xor = p[p.len() - 1] ^ arr[r];
        ans += cnt.get(&xor).unwrap_or(&0);
        p.push(xor);
        *cnt.entry(xor).or_insert(0) += 1;
    }
    println!("{}", ans);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
