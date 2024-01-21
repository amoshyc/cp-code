#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut arr = vec![];
    for _ in 0..m {
        let _ = read::<usize>();
        let mut inp = readv::<usize>();
        inp.reverse();
        arr.push(inp);
    }

    let mut mem = vec![!0; n + 1];
    let mut que = std::collections::VecDeque::new();
    for i in 0..m {
        que.push_back(i);
    }

    while let Some(i) = que.pop_front() {
        let x = *arr[i].last().unwrap();
        if mem[x] == !0 {
            mem[x] = i;
        } else {
            arr[i].pop();
            arr[mem[x]].pop();
            if arr[i].len() > 0 {
                que.push_back(i);
            }
            if arr[mem[x]].len() > 0 {
                que.push_back(mem[x]);
            }
            mem[x] = !0;
        }
    }

    if (0..m).all(|i| arr[i].len() == 0) {
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
