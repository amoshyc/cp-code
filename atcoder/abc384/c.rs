#![allow(unused)]

fn main() {
    let scores = readv::<i64>();

    let mut people = [
        "ABCDE", "BCDE", "ACDE", "ABDE", "ABCE", "ABCD", "CDE", "BDE", "ADE", "BCE", "ACE", "BCD",
        "ABE", "ACD", "ABD", "ABC", "DE", "CE", "BE", "CD", "AE", "BD", "AD", "BC", "AC", "AB",
        "E", "D", "C", "B", "A",
    ];

    people.sort_by_key(|s| {
        let mut v = 0;
        for c in s.chars() {
            v += scores[c as usize - 'A' as usize];
        }
        (-v, *s)
    });

    println!("{}", join(&people, "\n"));
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
