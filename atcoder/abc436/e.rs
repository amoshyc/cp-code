#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        nxt: [Usize1; n],
    }

    let mut ans = 0;
    for cycle in find_cycles_in_permutation_graph(&nxt) {
        let s = cycle.len() as i64;
        ans += s * (s - 1) / 2;
    }

    println!("{ans}");
}

fn find_cycles_in_permutation_graph(nxt: &Vec<usize>) -> Vec<Vec<usize>> {
    let n = nxt.len();
    let mut idx = vec![!0; n];
    let mut cycles = vec![];
    for r in 0..n {
        if idx[r] == !0 {
            idx[r] = 0;
            let mut path = vec![r];
            let mut u = nxt[r];
            while idx[u] == !0 {
                idx[u] = path.len();
                path.push(u);
                u = nxt[u];
            }
            cycles.push(path);
        }
    }
    cycles
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
