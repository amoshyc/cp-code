#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let edge = readv::<usize>();
        let (u, v) = (edge[0], edge[1]);
        adj[u].push(v);
    }

    let (num_scc, belong) = tarjan_scc(&adj);
    let mut sccs = vec![vec![]; num_scc];
    for u in 0..n {
        sccs[belong[u]].push(u);
    }

    let mut ans = vec![format!("{}", num_scc)];
    for scc in sccs.iter().rev() {
        ans.push(format!("{} {}", scc.len(), join(&scc, " ")));
    }
    println!("{}", join(&ans, "\n"));
}

struct TarjanSCC {
    order: usize,
    index: Vec<usize>,
    lowlink: Vec<usize>,
    onstack: Vec<bool>,
    stack: Vec<usize>,
    scc_id: usize,
    belong: Vec<usize>,
}

// https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
// Returns:
//    num_scc: number of scc
//    belong: which scc each vertex belongs to
// The order of scc is a *reversed* topological sort of the DAG.
fn tarjan_scc(adj: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut data = TarjanSCC {
        order: 0,
        index: vec![!0; n],
        lowlink: vec![0; n],
        onstack: vec![false; n],
        stack: vec![],
        scc_id: 0,
        belong: vec![!0; n],
    };
    for root in 0..data.index.len() {
        if data.index[root] == !0 {
            tarjan_dfs(root, &mut data, adj);
        }
    }
    (data.scc_id, data.belong)
}

fn tarjan_dfs(u: usize, data: &mut TarjanSCC, adj: &Vec<Vec<usize>>) {
    data.index[u] = data.order;
    data.lowlink[u] = data.order;
    data.order += 1;
    data.stack.push(u);
    data.onstack[u] = true;

    for &v in adj[u].iter() {
        if data.index[v] == !0 {
            tarjan_dfs(v, data, adj);
            data.lowlink[u] = data.lowlink[u].min(data.lowlink[v]);
        } else if data.onstack[v] {
            data.lowlink[u] = data.lowlink[u].min(data.index[v]);
        }
    }

    if data.lowlink[u] == data.index[u] {
        while let Some(x) = data.stack.pop() {
            data.onstack[x] = false;
            data.belong[x] = data.scc_id;
            if x == u {
                break;
            }
        }
        data.scc_id += 1;
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
