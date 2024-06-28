#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (h, w) = (inp[0], inp[1]);
        let mut arr = vec![];
        for _ in 0..h {
            arr.push(reads());
        }

        let mut max_l = 1;
        let mut ans = (0, 0);

        for r in 0..h {
            for (x, l, c) in rle(&arr[r]) {
                if x == '#' {
                    if l >= max_l {
                        max_l = l;
                        ans = (r, c + l / 2);
                    }
                }
            }
        }

        for c in 0..w {
            let col: Vec<_> = (0..h).map(|r| arr[r][c]).collect();
            for (x, l, r) in rle(&col) {
                if x == '#' {
                    if l >= max_l {
                        max_l = l;
                        ans = (r + l / 2, c);
                    }
                }
            }
        }

        println!("{} {}", ans.0 + 1, ans.1 + 1);
    }
}

fn rle<T: Copy + PartialEq>(arr: &Vec<T>) -> Vec<(T, usize, usize)> {
    let n = arr.len();
    let mut res = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && arr[j] == arr[i] {
            j += 1;
        }
        res.push((arr[i], j - i, i));
        i = j;
    }
    res
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
