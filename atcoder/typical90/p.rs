#![allow(unused)]

// [Problem]
// A yen coin, B yen coin, and C yen coin are each used zero or more times to pay exactly N yen.
// Please find the minimum value that can be considered as the number of coins used.
// However, there are infinite numbers of each coin.
// You can pay exactly N yen using a total of 9999 or fewer coins.

// [Solution]
// The problem gurantee the answer < 10^4
// We can enumerate the number of A yen coin and the number of B yen coin in 10^8.
// The number of C yen coin can be found in O(1).

fn main() {
    let n = read::<i64>();
    let inp = readv::<i64>();
    let (a, b, c) = (inp[0], inp[1], inp[2]);

    let mut ans = 9999;
    for na in (0..).take_while(|&na| a * na <= n && na <= 9999) {
        for nb in (0..).take_while(|&nb| a * na + b * nb <= n && na + nb <= 9999) {
            let remain = n - a * na - b * nb;
            if remain >= 0 && remain % c == 0 {
                let nc = remain / c;
                if na + nb + nc <= 9999 {
                    ans = ans.min(na + nb + nc);
                }
            }
        }
    }

    println!("{}", ans);
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
