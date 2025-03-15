#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut p2g = (0..n).collect::<Vec<usize>>(); // pigeon -> gid
    let mut n2g = (0..n).collect::<Vec<usize>>(); // nest -> gid
    let mut g2n = (0..n).collect::<Vec<usize>>(); // gid -> nest

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let a = ask[1] - 1;
            let nb = ask[2] - 1;
            p2g[a] = n2g[nb];
        } else if ask[0] == 2 {
            let na = ask[1] - 1;
            let nb = ask[2] - 1;
            let ga = n2g[na];
            let gb = n2g[nb];
            n2g.swap(na, nb);
            g2n.swap(ga, gb);
        } else {
            let a = ask[1] - 1;
            ans.push(g2n[p2g[a]] + 1);
        }
    }

    println!("{}", join(&ans, "\n"));
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
