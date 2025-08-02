#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut x = vec![0.0; n];
    let mut y = vec![0.0; n];
    for i in 0..n {
        let inp = readv::<f64>();
        x[i] = inp[0];
        y[i] = inp[1];
    }

    let mut total = 0.0;
    let mut cnt = 0;
    for perm in perm_iter(n) {
        let dis: f64 = perm
            .windows(2)
            .map(|w| {
                let (u, v) = (w[0], w[1]);
                ((x[u] - x[v]).powi(2) + (y[u] - y[v]).powi(2)).sqrt()
            })
            .sum();
        total += dis;
        cnt += 1;
    }

    println!("{:.7}", total / (cnt as f64));
}

fn next_perm<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
}

fn perm_iter(n: usize) -> impl std::iter::Iterator<Item = Vec<usize>> {
    let mut perm: Vec<usize> = (0..n).collect();
    let iter1 = std::iter::once(perm.clone());
    let iter2 = std::iter::from_fn(move || next_perm(&mut perm).and_then(|_| Some(perm.clone())));
    iter1.chain(iter2)
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
