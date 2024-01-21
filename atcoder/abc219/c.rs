#![allow(unused)]

fn main() {
    let order = reads();
    let mut mapping = std::collections::HashMap::new();
    for (i, &c) in order.iter().enumerate() {
        let x = i as u32 + 'a' as u32;
        mapping.insert(c, std::char::from_u32(x).unwrap());
    }

    let n: usize = read();
    let mut arr = vec![];
    let mut key = vec![];
    for _ in 0..n {
        let s = reads();
        key.push(join(&mapv(&s, |c| mapping[c]), ""));
        arr.push(s);
    }
    
    let mut idxs: Vec<usize> = (0..n).collect();
    idxs.sort_by_key(|&i| key[i].clone());

    let arr = mapv(&idxs, |&i| join(&arr[i], ""));
    println!("{}", join(&arr, "\n"));
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
