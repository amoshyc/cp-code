#![allow(unused)]

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [(usize, usize); n],
    }

    let mut vert_id = vec![0; n + 1];
    let mut children = vec![BTreeMap::<usize, usize>::new(); n + 1];
    let mut inverse = vec![];

    vert_id[0] = 0;
    inverse.push(vec![0]);

    for (i, &(par_seq, item)) in arr.iter().enumerate() {
        let seq_id = i + 1;
        let par_v = vert_id[par_seq];

        if let Some(&u) = children[par_v].get(&item) {
            // vert already existed
            inverse[u].push(seq_id);
            vert_id[seq_id] = u;
        } else {
            // vert not existed
            let u = inverse.len();
            children[par_v].insert(item, u);
            inverse.push(vec![seq_id]);
            vert_id[seq_id] = u;
        }
    }

    let mut ans = vec![];
    dfs(0, &children, &inverse, &mut ans);

    println!("{}", join(&ans[1..], " "));
}

fn dfs(
    u: usize,
    children: &Vec<BTreeMap<usize, usize>>,
    inverse: &Vec<Vec<usize>>,
    ans: &mut Vec<usize>,
) {
    ans.extend(inverse[u].clone());
    for (_, &v) in &children[u] {
        dfs(v, children, inverse, ans);
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
