#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr_p = readv::<usize>();
    let arr_q = readv::<usize>();

    let arr_p = mapv(&arr_p, |&x| x - 1);
    let arr_q = mapv(&arr_q, |&x| x - 1);

    let mut a = 0;
    let mut b = 0;
    for (i, perm) in perm_iter(n).enumerate() {
        if perm == arr_p {
            a = i;
        }
        if perm == arr_q {
            b = i;
        }
    }

    println!("{}", a.abs_diff(b));
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
