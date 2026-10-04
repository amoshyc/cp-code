#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        cost: [[usize; n]; n],
    }

    let count_min_swaps = |perm: &Vec<usize>| -> usize {
        let mut dsu = DSU::new(n);
        for i in 0..n {
            dsu.unite(i, perm[i]);
        }

        let mut cnt = 0;
        for i in 0..n {
            if dsu.root(i) == i {
                cnt += dsu.size(i) - 1;
            }
        }

        cnt
    };

    let mut ans = 0;
    for perm in perm_iter(n) {
        if count_min_swaps(&perm) <= k {
            let mut total = cost[perm[n - 1]][perm[0]];
            for w in perm.windows(2) {
                total += cost[w[0]][w[1]];
            }

            ans = ans.max(total);
        }
    }

    println!("{}", ans);
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

struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, u: usize) -> usize {
        if self.par[u] == u {
            u
        } else {
            self.par[u] = self.root(self.par[u]);
            self.par[u]
        }
    }

    fn unite(&mut self, mut u: usize, mut v: usize) {
        u = self.root(u);
        v = self.root(v);
        if u == v {
            return;
        }
        if self.siz[u] > self.siz[v] {
            self.par[v] = u;
            self.siz[u] += self.siz[v];
        } else {
            self.par[u] = v;
            self.siz[v] += self.siz[u];
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, u: usize) -> usize {
        let r = self.root(u);
        self.siz[r]
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
