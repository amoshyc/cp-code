#![allow(unused)]

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // build tokens
    let mut tokens = vec![];
    for i in (0..).take_while(|&i| (1 << i) <= 1_000_000_000) {
        let token = (1 << i)
            .to_string()
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<i32>>();
        tokens.push(token);
    }

    // Find all good integers
    let mut result = vec![];
    dfs(0, true, 0, &tokens, &mut result);
    dfs(0, false, 0, &tokens, &mut result);
    result.sort();
    result.dedup();

    println!("{}", result[n - 1]);
}

fn dfs(i: usize, leading0: bool, value: i32, tokens: &Vec<Vec<i32>>, result: &mut Vec<i32>) {
    if value >= 1_000_000_000 {
        return;
    }

    if i == 10 {
        result.push(value);
        return;
    }

    // leading zero
    if leading0 && i < 9 {
        dfs(i + 1, true, value, tokens, result);
    }

    // chose a feasible token
    for token in tokens.iter() {
        if i + token.len() <= 10 {
            let mut new_value = value;
            for d in token.iter() {
                new_value = new_value.saturating_mul(10);
                new_value = new_value.saturating_add(*d);
            }

            dfs(i + token.len(), false, new_value, tokens, result);
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
