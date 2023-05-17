use std::collections::HashSet;

type Set = HashSet<i32>;

fn main() {
    let inp = readv::<i32>();
    let (x, y) = (inp[1], inp[2]);
    let arr = readv::<i32>();

    let mut set_x = Set::new();
    let mut set_y = Set::new();

    set_x.insert(arr[0]);
    set_y.insert(0);
    
    for a in arr[2..].iter().step_by(2) {
        let mut next_set = Set::new();
        for p in set_x.iter() {
            next_set.insert(p + a);
            next_set.insert(p - a);
        }
        set_x = next_set;
    }
    for a in arr[1..].iter().step_by(2) {
        let mut next_set = Set::new();
        for p in set_y.iter() {
            next_set.insert(p + a);
            next_set.insert(p - a);
        }
        set_y = next_set;
    }

    if set_x.contains(&x) && set_y.contains(&y) {
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

// fn join<T: ToString>(v: &[T], sep: &str) -> String {
//     v.iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>()
//         .join(sep)
// }
