fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut tree = UnionFind::new(n);
    let mut ans = vec![];
    for _ in 0..q {
        let cmd = readv::<usize>();
        let (t, u, v) = (cmd[0], cmd[1], cmd[2]);
        if t == 0 {
            tree.unite(u, v);
        } else {
            ans.push(tree.same(u, v) as i32);
        }
    }
    println!("{}", join(&ans, "\n"));
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: vec![n; n],
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == self.parent.len() {
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

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return ()
        }
        if self.size(ra) > self.size(rb) {
            // (a, b) = (b, a);
            let r = ra;
            ra = rb;
            rb = r;
        }
        self.parent[ra] = rb;
        self.size[rb] += self.size[ra];
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
