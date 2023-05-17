#![allow(unused)]

fn main() {
    let s = reads();

    let ok1 = s[0].is_uppercase();
    let ok2 = s[s.len() - 1].is_uppercase();
    let ok3 = s.len() == 8;

    let mut ok4 = false;
    if ok1 && ok2 && ok3 {
        let num = s[1..s.len() - 1].iter().collect::<String>().parse::<i32>();
        ok4 = num.is_ok() && (100000..=999999).contains(&num.unwrap());
    }

    if ok1 && ok2 && ok3 && ok4 {
        println!("Yes");
    } else {
        println!("No");
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

fn split<T: Clone, F: FnMut(&T) -> bool>(arr: &[T], pred: F) -> Vec<Vec<T>> {
    arr.split(pred).map(|x| x.to_vec()).collect()
}