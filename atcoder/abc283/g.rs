#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, l, r) = (inp[0], inp[1] - 1, inp[2] - 1);
    let arr = readv::<u64>();

    let mut basis: Vec<u64> = vec![];
    for &x in arr.iter() {
        let mut x = x;
        for &b in basis.iter() {
            x = x.min(x ^ b);
        }
        if x > 0 {
            for i in 0..basis.len() {
                basis[i] = basis[i].min(basis[i] ^ x);
            }
            basis.push(x);
        }
    }

    basis.sort();

    let mut ans = vec![];
    for i in l..=r {
        // the i-th smallest in span(basis)
        let mut x = 0;
        for (j, &b) in basis.iter().enumerate() {
            if i & (1 << j) != 0 {
                x ^= b;
            }
        }
        ans.push(x);
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
