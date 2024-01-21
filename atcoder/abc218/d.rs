#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut pts = vec![];
    let mut set = std::collections::HashSet::new();
    for _ in 0..n {
        let inp = readv::<i32>();
        let (x, y) = (inp[0], inp[1]);
        pts.push((x, y));
        set.insert((x, y));
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[j];
            if x1 == x2 || y1 == y2 {
                continue;
            }
            if set.contains(&(x1, y2)) && set.contains(&(x2, y1)) {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt / 2);
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
