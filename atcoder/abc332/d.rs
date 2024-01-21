#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut arr1 = vec![];
    let mut arr2 = vec![];
    for r in 0..n {
        arr1.push(readv::<usize>());
    }
    for r in 0..n {
        arr2.push(readv::<usize>());
    }

    let mut que = std::collections::VecDeque::new();
    let mut vis = std::collections::HashSet::new();

    que.push_back((arr1.clone(), 0));
    vis.insert(arr1);

    while let Some((arr, dis)) = que.pop_front() {
        if arr == arr2 {
            println!("{}", dis);
            return;
        }

        let mut nxt = arr.clone();

        for r in 0..(n - 1) {
            nxt.swap(r, r + 1);
            if !vis.contains(&nxt) {
                que.push_back((nxt.clone(), dis + 1));
                vis.insert(nxt.clone());
            }
            nxt.swap(r, r + 1);
        }

        for c in 0..(m - 1) {
            for r in 0..n {
                nxt[r].swap(c, c + 1);
            }
            if !vis.contains(&nxt) {
                que.push_back((nxt.clone(), dis + 1));
                vis.insert(nxt.clone());
            }
            for r in 0..n {
                nxt[r].swap(c, c + 1);
            }
        }
    }

    println!("-1");
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
