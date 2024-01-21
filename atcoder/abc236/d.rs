#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut aff = vec![vec![0; 2 * n]; 2 * n];

    for r in 0..(2 * n - 1) {
        let inp = readv::<u64>();
        for c in 0..inp.len() {
            aff[r][r + 1 + c] = inp[c];
            aff[r + 1 + c][r] = inp[c];
        }
    }

    let mut dfs = DFS {
        ans: 0,
        seq: vec![!0; 2 * n],
        aff,
    };
    dfs.run(0, 0);

    println!("{}", dfs.ans);
}

struct DFS {
    ans: u64,
    seq: Vec<usize>,
    aff: Vec<Vec<u64>>,
}

impl DFS {
    fn run(&mut self, i: usize, mask: usize) {
        if i == self.aff.len() {
            let mut xor = 0;
            for i in (1..self.seq.len()).step_by(2) {
                xor = xor ^ self.aff[self.seq[i - 1]][self.seq[i]];
            }
            self.ans = self.ans.max(xor);
            return;
        }

        let m = self.aff.len();
        if i % 2 == 0 {
            let j = (0..m).filter(|&j| (mask >> j) & 1 == 0).next().unwrap();
            self.seq[i] = j;
            self.run(i + 1, mask | (1 << j));
            self.seq[i] = !0;
        } else {
            for j in 0..m {
                if (mask >> j) & 1 == 0 {
                    self.seq[i] = j;
                    self.run(i + 1, mask | (1 << j));
                    self.seq[i] = !0;
                }
            }
        }
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
