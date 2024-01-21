#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let arr = readv::<u64>();

    let mut larger = BTreeSet::new();
    let mut smaller = BTreeSet::new();
    let mut sum = 0;
    for i in 0..m {
        smaller.insert((arr[i], i));
        sum += arr[i];
    }
    while smaller.len() > k {
        let &(x, i) = smaller.iter().next_back().unwrap();
        larger.insert((x, i));
        smaller.remove(&(x, i));
        sum -= x;
    }
    
    let mut ans = vec![sum];
    for e in m..n {
        // Remove arr[s]
        let s = e - m;
        if larger.contains(&(arr[s], s)) {
            larger.remove(&(arr[s], s));
        } else {
            smaller.remove(&(arr[s], s));
            sum -= arr[s];
        }
        
        // Add arr[e]
        let &(pivot, _) = smaller.iter().next_back().unwrap_or(&(0, 0));
        if arr[e] > pivot {
            larger.insert((arr[e], e));
        } else {
            smaller.insert((arr[e], e));
            sum += arr[e];
        }
        
        // Balance
        while smaller.len() > k {
            let &(x, i) = smaller.iter().next_back().unwrap();
            larger.insert((x, i));
            smaller.remove(&(x, i));
            sum -= x;
        }
        while smaller.len() < k {
            let &(x, i) = larger.iter().next().unwrap();
            smaller.insert((x, i));
            sum += x;
            larger.remove(&(x, i));
        }
        
        // Update answer
        ans.push(sum);
    }

    println!("{}", join(&ans, " "));
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
