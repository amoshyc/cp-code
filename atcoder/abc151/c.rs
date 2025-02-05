#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut groups = vec![vec![]; n];
    for i in 0..m {
        let inp = readv::<String>();
        let p = inp[0].parse::<usize>().unwrap() - 1;
        let v = inp[1].chars().next().unwrap();
        groups[p].push(v);
    }

    let mut cnt_ac = 0;
    let mut cnt_wa = 0;
    for i in 0..n {
        if let Some(p) = groups[i].iter().position(|x| *x == 'A') {
            cnt_ac += 1;
            cnt_wa += p;
        }
    }

    println!("{} {}", cnt_ac, cnt_wa);
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
    read::<String>().chars().collect()
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
