#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut ingr = vec![vec![]; m];
    for i in 0..m {
        let inp = readv::<usize>();
        for x in &inp[1..] {
            ingr[i].push(x - 1);
        }
    }

    let b = readv::<usize>();
    let mut t = vec![0; n]; // t[i] = the day ingr i is available
    for i in 0..n {
        t[b[i] - 1] = i;
    }

    let mut avail = vec![vec![]; n];
    for i in 0..m {
        let max_time = ingr[i].iter().map(|&i| t[i]).max().unwrap();
        avail[max_time].push(i);
    }

    let mut ans = vec![avail[0].len()];
    for i in 1..n {
        ans.push(ans[i - 1] + avail[i].len());
    }

    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
