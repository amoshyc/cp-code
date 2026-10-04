#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        arr_p: [usize; n],
        arr_q: [usize; n],
    }

    let perms = perm_iter(n).collect::<Vec<_>>();

    let arr_p = arr_p.iter().map(|&x| x - 1).collect();
    let arr_q = arr_q.iter().map(|&x| x - 1).collect();

    let idx_p = perms.binary_search(&arr_p).unwrap();
    let idx_q = perms.binary_search(&arr_q).unwrap();

    if idx_p < idx_q {
        println!("{}", idx_q - idx_p - 1);
    } else {
        println!("0");
    }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
