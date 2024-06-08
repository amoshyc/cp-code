#![allow(unused)]

// [Problem]
// There is a grid of H rows and W columns, and we denote the cell in the i-th row from the top and the j-th column from the left as (i,j).
// Initially, all cells are white. Here, Q queries are given in the following format.
//
// For the i-th query (1<=i<=Q):
// ti=1:
//     An integer ri, ci is given.
//     The white cell (ri,ci) is painted red.
// ti=2:
//     Integers rai, cai, rbi, cbi are given.
//     Output Yes if both of the following conditions are satisfied; otherwise, output No.
//         1. The cells (rai,cai) and (rbi,cbi) are painted red.
//         2. It is possible to reach from the cell (rai,cai) to the cell (rbi,cbi) by moving vertically or horizontally along the red cells.
//
// Please process the above Q queries sequentially.

// [Solution]
// Store the red clusters using DSU.
// Cell (r, c) is node r * w + c in DSU.
// For ti = 1, merge neighboring cells with (r, c) if they are both red.
// For ti = 2, simply check if (ra, ca) belongs to same group as (rb, cb).

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let q = read::<usize>();

    let mut dsu = DSU::new(h * w);
    let mut color = vec![vec!['w'; w]; h];

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();

        if ask[0] == 1 {
            let (r, c) = (ask[1] - 1, ask[2] - 1);
            color[r][c] = 'r';
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(!0);
                let nc = c.checked_add_signed(dc).unwrap_or(!0);
                if nr < h && nc < w && color[nr][nc] == 'r' {
                    dsu.unite(r * w + c, nr * w + nc);
                }
            }
        } else {
            let (ra, ca) = (ask[1] - 1, ask[2] - 1);
            let (rb, cb) = (ask[3] - 1, ask[4] - 1);
            let mut ok = true;
            ok &= color[ra][ca] == 'r';
            ok &= color[rb][cb] == 'r';
            ok &= dsu.same(ra * w + ca, rb * w + cb);
            if ok {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        }
    }

    println!("{}", join(&ans, "\n"));
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
    read::<String>().chars().collect::<_>()
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
