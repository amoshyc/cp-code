#![allow(unused)]

fn main() {
    let s = reads();
    let s = mapv(&s, |c| c.to_digit(10).unwrap() as usize);
    let n = s.len();

    let mut pow10 = vec![1];
    for i in 1..=n {
        pow10.push(pow10[i - 1] * 10 % 2019);
    }

    // S[l..r] is multiple of 2019 <-> case 1 or case 2 where
    //    case1: int(S[l..]) = int(S[r..]) (mod 2019)
    //    case2: int(S[l..]) = 0 (mod 2019)
    // => Count the number of each remainder for all the suffixes.
    let mut ans = 0;
    let mut cnt = vec![0i64; 2019];
    let mut suff = 0;
    for i in (0..n).rev() {
        suff = (s[i] * pow10[n - i - 1] + suff) % 2019;
        cnt[suff] += 1;
        if suff == 0 {
            ans += 1;
        }
    }

    for r in 0..2019 {
        if cnt[r] >= 2 {
            ans += cnt[r] * (cnt[r] - 1) / 2;
        }
    }

    println!("{}", ans);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T1: std::str::FromStr, T2: std::str::FromStr>() -> (T1, T2) {
    let inp = read::<String>();
    let inp: Vec<_> = inp.split_ascii_whitespace().collect();
    let a: T1 = inp[0].parse().ok().unwrap();
    let b: T2 = inp[1].parse().ok().unwrap();
    (a, b)
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
