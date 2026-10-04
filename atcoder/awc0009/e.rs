#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        arr: [i64; n],
        ask: [(Usize1, Usize1); q],
    }

    let max_st = SparseTable::<i64>::new(&arr, |a, b| a.max(b));
    let min_st = SparseTable::<i64>::new(&arr, |a, b| a.min(b));

    let mut ans = vec![0; q];
    for (i, &(l, r)) in ask.iter().enumerate() {
        let max = max_st.query(l, r + 1);
        let min = min_st.query(l, r + 1);
        ans[i] = max - min;
    }

    println!("{}", join(&ans, "\n"));
}

struct SparseTable<T> {
    dp: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}

impl<T: Copy> SparseTable<T> {
    fn new(arr: &Vec<T>, op: fn(T, T) -> T) -> Self {
        let nn = (arr.len() as f64).log2() as usize + 1;
        let mut dp = vec![arr.clone(); nn];
        for i in 1..nn {
            for u in 0..=(arr.len() - (1 << i)) {
                dp[i][u] = (op)(dp[i - 1][u], dp[i - 1][u + (1 << (i - 1))]);
            }
        }
        Self { dp, op }
    }

    // a..b
    fn query(&self, a: usize, b: usize) -> T {
        assert!(a != b);
        let k = ((b - a) as f64).log2() as usize;
        (self.op)(self.dp[k][a], self.dp[k][b - (1 << k)])
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
