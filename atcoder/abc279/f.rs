fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let none = std::usize::MAX;
    let mut num_ball = n;
    let mut dsu = UnionFind::new(n + q);
    let mut pos = vec![none; n + q]; // root_ball_id -> box_id
    let mut any = vec![none; n]; // box_id -> ball_id
    for i in 0..n {
        pos[i] = i;
        any[i] = i;
    }

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let cmd = inp[0];

        if cmd == 1 {
            let (x, y) = (inp[1] - 1, inp[2] - 1);
            if any[y] != none {
                if any[x] != none {
                    dsu.unite(any[x], any[y]);
                }
                any[x] = any[y];
                any[y] = none;
                pos[dsu.root(any[x])] = x;
            }
        } else if cmd == 2 {
            let x = inp[1] - 1;
            let b = num_ball;
            num_ball += 1;
            if any[x] != none {
                dsu.unite(any[x], b);
            }
            any[x] = b;
            pos[dsu.root(b)] = x;
        } else {
            let x = inp[1] - 1;
            ans.push(pos[dsu.root(x)] + 1);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.root(self.parent[x]);
            self.parent[x]
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.size[x]
    }

    // fn same(&mut self, a: usize, b: usize) -> bool {
    //     self.root(a) == self.root(b)
    // }

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return;
        }
        if self.size(a) > self.size(b) {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[a] = b;
        self.size[b] += self.size[a];
    }
}