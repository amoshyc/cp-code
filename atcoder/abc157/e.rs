#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut s = reads();

    let id = |c: char| c as usize - 'a' as usize;

    let mut bits = vec![BIT::<i32>::new(n); 26];
    for i in 0..n {
        bits[id(s[i])].add(i, 1);
    }

    let mut ans = vec![];
    let q = read::<usize>();
    for qid in 0..q {
        let ask = readv::<String>();
        if ask[0] == "1" {
            let i = ask[1].parse::<usize>().unwrap() - 1;
            let c = ask[2].chars().next().unwrap();
            if s[i] != c {
                bits[id(s[i])].add(i, -1);
                s[i] = c;
                bits[id(s[i])].add(i, 1);
            }
        } else {
            let mut val = 0;
            let l = ask[1].parse::<usize>().unwrap() - 1;
            let r = ask[2].parse::<usize>().unwrap() - 1;
            for c in 0..26 {
                if bits[c].sum(l, r + 1) > 0 {
                    val += 1;
                }
            }
            ans.push(val);
        }
    }

    println!("{}", join(&ans, "\n"));
}

#[derive(Clone)]
struct BIT<T> {
    dat: Vec<T>,
}

impl<T: Clone + Default + std::ops::AddAssign + std::ops::Sub<Output = T>> BIT<T> {
    fn new(n: usize) -> Self {
        Self {
            dat: vec![T::default(); n + 1],
        }
    }

    // 0-based
    fn add(&mut self, mut i: usize, x: T) {
        i += 1; // convert to 1-based
        while i < self.dat.len() {
            self.dat[i] += x.clone();
            i += i & (!i + 1); // i & (-i)
        }
    }

    // 0..=i, 0-based
    fn pref(&self, mut i: usize) -> T {
        let mut res = T::default();
        i += 1; // convert to 1-based
        while i > 0 {
            res += self.dat[i].clone();
            i -= i & (!i + 1);
        }
        res
    }

    // l..i, 0-based
    fn sum(&self, mut l: usize, mut r: usize) -> T {
        if r == 0 {
            T::default()
        } else if l >= 1 {
            self.pref(r - 1) - self.pref(l - 1)
        } else {
            self.pref(r - 1)
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
