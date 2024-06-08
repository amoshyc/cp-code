#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, d) = (inp[0] as usize, inp[1]);
    let mut adj = vec![vec![]; 2 * n];

    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        let inp = readv::<i64>();
        x[i] = inp[0];
        y[i] = inp[1];
    }

    // 2-SAT:
    // (p or q) => add edge (~p => q), (~q => p)
    // (p xor q) = (~p or ~q) and (p or q)
    // p or (q and r) = (p or q) and (p or r)
    // After scc, p is true if tps[p] > tps[~p] where tps is topological sort.
    // That is, p is true if belong[p] < belong[~p]

    let mut add_clause = |p: usize, q: usize| {
        adj[p ^ 1].push(q);
        adj[q ^ 1].push(p);
    };

    // flag i has 2 nodes in graph:
    //   pos(i) = node 2 * i = flag i choose x[i]
    //   neg(i) = node 2 * i + 1 = flag i choose y[i]

    for i in 0..n {
        for j in (i + 1)..n {
            let (pos_i, neg_i) = (2 * i, 2 * i + 1);
            let (pos_j, neg_j) = (2 * j, 2 * j + 1);
            if (x[i] - x[j]).abs() < d {
                // We cannot choose both x[i] and x[j]
                // ~(pos_i and pos_j) = (neg_i or neg_j)
                add_clause(neg_i, neg_j);
            }
            if (x[i] - y[j]).abs() < d {
                // We cannot choose both x[i] and y[j]
                // ~(pos_i and neg_j) = (neg_i or pos_j)
                add_clause(neg_i, pos_j);
            }
            if (y[i] - x[j]).abs() < d {
                // We cannot choose both y[i] and x[j]
                // ~(neg_i and pos_j) = (pos_i or neg_j)
                add_clause(pos_i, neg_j);
            }
            if (y[i] - y[j]).abs() < d {
                // We cannot choose both y[i] and y[j]
                // ~(neg_i and neg_j) = (pos_i or pos_j)
                add_clause(pos_i, pos_j);
            }
        }
    }

    let (num_scc, belong) = tarjan_scc(&adj);

    // Check contradiction
    for i in 0..n {
        let pos_i = 2 * i;
        if belong[pos_i] == belong[pos_i ^ 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
    let mut ans = vec![];
    for i in 0..n {
        let pos_i = 2 * i;
        if belong[pos_i] < belong[pos_i ^ 1] {
            ans.push(x[i]);
        } else {
            ans.push(y[i]);
        }
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
