#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    if k <= n - k {
        let mut ans = 0;
        for comb in comb_iter(n, k) {
            let xor = comb.iter().fold(0, |acc, i| acc ^ arr[*i]);
            ans = ans.max(xor);
        }
        println!("{}", ans);
    } else {
        let mut ans = 0;
        let all = arr.iter().fold(0, |acc, x| acc ^ x);
        for comb in comb_iter(n, n - k) {
            let xor = comb.iter().fold(0, |acc, i| acc ^ arr[*i]);
            ans = ans.max(all ^ xor);
        }
        println!("{}", ans);
    }
}

fn next_comb(comb: &mut Vec<usize>, n: usize, r: usize) -> Option<()> {
    let i = (0..r).rposition(|j| comb[j] != j + n - r)?;
    comb[i] += 1;
    for j in (i + 1)..r {
        comb[j] = comb[j - 1] + 1;
    }
    Some(())
}

fn comb_iter(n: usize, r: usize) -> impl std::iter::Iterator<Item = Vec<usize>> {
    let mut comb: Vec<usize> = (0..r).collect();
    let iter1 = std::iter::once(comb.clone());
    let iter2 =
        std::iter::from_fn(move || next_comb(&mut comb, n, r).and_then(|_| Some(comb.clone())));
    iter1.chain(iter2)
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
