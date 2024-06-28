#![allow(unused)]

fn main() {
    // 2-SAT:
    // (p or q) => add edge (~p => q), (~q => p)
    // (p) = (p or p)
    // (p xor q) = (~p or ~q) and (p or q)
    // (p or (q and r)) = (p or q) and (p or r)
    // At least 1 of (p, q) is true: (p or q)
    // At least 2 of (p, q) is true: (p and q)
    // At least 1 of (p, q, r) is true: (p or q or r) => Not a CNF.
    // At least 2 of (p, q, r) is true: (p or q) and (q or r) and (p or r)
    // At least 3 of (p, q, r) is true: (p and q and r)
    // After scc,
    //     * No solution if any p has belong[p] = belong[~p].
    //     * p is true if belong[p] < belong[~p], i.e., topo(p) > topo(~p).

    // p q   eval
    // - -    F
    // - +    T
    // + -    F
    // + +    T
    // If we get a truth table, we can construct the CNF.
    // We ban all the entries that are F. The DNF of banned entries is
    //    (~p and ~q) or (p and ~q)
    // Negate the DNF and we get CNF of the T entries:
    //    (p or q) and (~p or q)

    let mut ans = vec![];

    let tc = read::<usize>();
    for _ in 0..tc {
        let n = read::<usize>();
        let mut adj = vec![vec![]; 2 * n];
        let mut add_clause = |p: usize, q: usize| {
            adj[p ^ 1].push(q);
            adj[q ^ 1].push(p);
        };

        let get_node = |x: i32| if x > 0 { 2 * (x - 1) } else { 2 * (-x - 1) + 1 };
        let mut arr = vec![readv::<i32>(), readv::<i32>(), readv::<i32>()];
        for c in 0..n {
            let p = get_node(arr[0][c]) as usize;
            let q = get_node(arr[1][c]) as usize;
            let r = get_node(arr[2][c]) as usize;
            add_clause(p, q);
            add_clause(q, r);
            add_clause(p, r);
        }

        let (_, belong) = tarjan_scc(&adj);
        let ok = (0..n).all(|i| belong[2 * i] != belong[2 * i ^ 1]);
        if ok {
            ans.push("Yes");
        } else {
            ans.push("No");
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
