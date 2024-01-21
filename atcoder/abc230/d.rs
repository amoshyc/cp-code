#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, d) = (inp[0], inp[1]);
    let mut segs = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        segs.push((inp[0], inp[1]));
    }
    segs.sort_by_key(|&(s, t)| (t, s));

    let mut cnt = 0;
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && segs[j].0 <= segs[i].1 + d - 1 {
            j += 1;
        }
        cnt += 1;
        i = j;
    }
    
    println!("{}", cnt);
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
