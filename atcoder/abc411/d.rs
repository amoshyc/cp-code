#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut nodes = vec![];
    let mut parent = vec![];

    nodes.push(vec![]);
    parent.push(0);

    let mut server = 0;
    let mut pc = vec![0; n];

    for _ in 0..q {
        let ask = readv::<String>();
        let cmd = ask[0].parse::<usize>().unwrap();
        if cmd == 1 {
            let p = ask[1].parse::<usize>().unwrap() - 1;
            pc[p] = server;
        } else if cmd == 2 {
            let p = ask[1].parse::<usize>().unwrap() - 1;
            let s = ask[2].chars().collect::<Vec<char>>();
            let node_id = nodes.len();
            nodes.push(s);
            parent.push(pc[p]);
            pc[p] = node_id;
        } else {
            let p = ask[1].parse::<usize>().unwrap() - 1;
            server = pc[p];
        }
    }

    let mut x = server;
    let mut ans = vec![];
    while x != 0 {
        let mut token = nodes[x].clone();
        token.reverse();
        ans.extend(token);
        x = parent[x];
    }
    ans.reverse();
    println!("{}", join(&ans, ""));
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
