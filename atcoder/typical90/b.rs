#![allow(unused)]

// [Problem]
// Output all valid bracket sequences of length N in lexicographical order.
// A valid bracket sequence is defined as follows:
//     () is a valid bracket sequence.
//     If S is a valid bracket sequence, then the string ( + S + ) is a valid bracket sequence.
//     If S and T are valid bracket sequences, then the string S + T is a valid bracket sequence.
//     Any other string is not a valid bracket sequence.
// For example:
//     ()()
//     (()())(())
//     ()()()()()()()()
// are valid bracket sequences, but
//     )(
//     )))()(((
//     ((((a))))
// are not valid bracket sequences.
// Also, ( is considered to be lexicographically smaller than ).

// [Solution]
// Since N is small, we can enumerate all the sequences consisting of ( and ) and rejects those that are invalid.
// By treating ( as 0 and ) as 1, all sequences are the binary bits of integers 0..2^N.

fn main() {
    let n = read::<usize>();
    let mut ans = vec![];
    for mask in 0..(1 << n) {
        let seq: Vec<char> = (0..n)
            .map(|i| if (mask >> i) & 1 == 0 { '(' } else { ')' })
            .collect();

        let mut pref = 0;
        let mut valid = true;
        for c in seq.iter() {
            pref += if *c == '(' { 1 } else { -1 };
            if pref < 0 {
                valid = false;
            }
        }
        valid &= pref == 0;

        if valid {
            ans.push(format!("{}", join(&seq, "")));
        }
    }

    ans.sort();
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
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
