#![allow(unused)]

fn main() {
    let s = reads();
    let t = reads();

    let rle_s = rle(&s);
    let rle_t = rle(&t);

    if rle_s.len() != rle_t.len() {
        println!("No");
        return;
    }

    for (&(c1, l1), &(c2, l2)) in rle_s.iter().zip(rle_t.iter()) {
        if c1 != c2 {
            println!("No");
            return;
        }
        if l1 < l2 {
            if l1 == 1 {
                println!("No");
                return;
            }
        }
        if l1 > l2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn rle(s: &Vec<char>) -> Vec<(char, usize)> {
    let n = s.len();
    let mut res = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && s[j] == s[i] {
            j += 1;
        }
        res.push((s[i], j - i));
        i = j;
    }
    res
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
