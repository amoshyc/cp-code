#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = readv::<i64>();
    
    let mut ans = 0;
    let mut cands = vec![];
    arr.push(0);
    for &x in arr.iter() {
        let mut w = 0;
        while cands.last().map_or(false, |&(a, l)| a >= x) {
            let (a, l) = cands.pop().unwrap();
            w += l;
            ans = ans.max(a * w);
        }
        cands.push((x, w + 1));
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
