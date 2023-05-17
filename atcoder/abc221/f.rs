fn main() {
    let mdl: u64 = 998244353;
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..n - 1 {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let (nodes, _, _) = bfs(&adj, 0, std::usize::MAX);
    let s = nodes[nodes.len() - 1];
    let (nodes, parent, _) = bfs(&adj, s, std::usize::MAX);
    let t = nodes[nodes.len() - 1];

    let mut path = vec![t];
    loop {
        let last = path[path.len() - 1];
        let &p = parent.get(&last).unwrap();
        if p != std::usize::MAX {
           path.push(p);
        } else {
            break;
        }
    }
    let diameter = path.len() - 1;

    if diameter % 2 == 0 {
        let root = path[diameter / 2];
        let mut cnts = vec![];
        for &child in adj[root].iter() {
            let (nodes, _, depth) = bfs(&adj, child, root);
            cnts.push(
                nodes
                    .iter()
                    .filter(|&&u| *depth.get(&u).unwrap() == diameter / 2 - 1)
                    .count() as u64,
            );
        }
        let mut ans = 1;
        for &cnt in cnts.iter() {
            ans = ans * (cnt + 1) % mdl;
        }
        for &cnt in cnts.iter() {
            ans = (ans - cnt + mdl) % mdl;
        }
        println!("{}", (ans - 1 + mdl) % mdl);
    } else {
        let (root1, root2) = (path[diameter / 2], path[diameter / 2 + 1]);
        let (nodes, _, depth) = bfs(&adj, root1, root2);
        let cnt1 = nodes
            .iter()
            .filter(|&&u| *depth.get(&u).unwrap() == (diameter - 1) / 2)
            .count() as u64;
        let (nodes, _, depth) = bfs(&adj, root2, root1);
        let cnt2 = nodes
            .iter()
            .filter(|&&u| *depth.get(&u).unwrap() == (diameter - 1) / 2)
            .count() as u64;
        let ans = (cnt1 % mdl) * (cnt2 % mdl) % mdl;
        println!("{}", ans);
    }
}

use std::collections::HashMap;
use std::collections::VecDeque;

fn bfs(
    adj: &Vec<Vec<usize>>,
    root: usize,
    par: usize,
) -> (Vec<usize>, HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut nodes = vec![];
    let mut parent = HashMap::new();
    let mut depth = HashMap::new();
    let mut queue = VecDeque::new();

    parent.insert(root, par);
    depth.insert(root, 0);

    queue.push_back((root, par));
    while !queue.is_empty() {
        let (u, p) = queue.pop_front().unwrap();
        nodes.push(u);
        for &v in adj[u].iter() {
            if v != p {
                parent.insert(v, u);
                depth.insert(v, *depth.get(&u).unwrap() + 1);
                queue.push_back((v, u));
            }
        }
    }
    (nodes, parent, depth)
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
