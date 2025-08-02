#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let s = reads();
    let mut t = s.clone();

    let mut cnt_o = 0;
    for i in 0..n {
        if s[i] == 'o' {
            cnt_o += 1;
            if i >= 1 && t[i - 1] == '?' {
                t[i - 1] = '.';
            }
            if i + 1 < n && t[i + 1] == '?' {
                t[i + 1] = '.';
            }
        }
    }
    let rem = k - cnt_o;

    if rem == 0 {
        for i in 0..n {
            if t[i] == '?' {
                t[i] = '.';
            }
        }
        println!("{}", join(&t, ""));
        return;
    }

    let segs = rle(&t);
    let mut cnt_even = 0;
    let mut cnt_odd = 0;
    for &(c, l, _) in &segs {
        if c == '?' {
            if l % 2 == 1 {
                cnt_odd += (l + 1) / 2;
            } else {
                cnt_even += l / 2;
            }
        }
    }

    // 7 0
    // ?...?..
    // .......

    // 7 2
    // .?.??..
    // .o.??..

    // 7 3
    // .?.???.
    // .o.o.o.

    // seg with even length => ????
    // seg with odd length => must be o.o.o

    if rem == cnt_even + cnt_odd {
        for &(c, l, i) in &segs {
            if c == '?' && l % 2 == 1 {
                for j in 0..l {
                    if j % 2 == 0 {
                        t[i + j] = 'o';
                    } else {
                        t[i + j] = '.';
                    }
                }
            }
        }
    }

    println!("{}", join(&t, ""));
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
    std::io::stdin().read_line(&mut s);
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
