#include <atcoder/lazysegtree>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint998244353;

struct S {
    mint value;
    mint base;
};
using F = int;

ostream& operator << (ostream& out, const S& data) {
    out << "[" << data.value.val() << " , " << data.base.val() << "]";
    return out;
}

S op(S a, S b) { return {a.value + b.value, a.base + b.base}; }
S default_data() { return {0, 0}; }
S apply_lazy(F lazy, S data) {
    if (lazy == 0) {
        return data;
    }
    return {lazy * data.base, data.base}; 
}
F push_lazy(F root, F child) {
    if (root == 0) {
        return child;
    }
    return root;
}
F default_lazy() { return 0; }

using SegTree = atcoder::lazy_segtree<S, op, default_data, F, apply_lazy,
                                      push_lazy, default_lazy>;


int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, Q;
    cin >> N >> Q;

    auto leaves = vector<S>(N);
    mint base = 1;
    for (int i = 0; i < N; i++) {
        leaves[i] = {base, base};
        base = base * 10;
    }
    auto tree = SegTree(leaves);

    while (Q--) {
        int l, r, d;
        cin >> l >> r >> d;
        l--; 
        r--;
        tree.apply(N - 1 - r, N - l, d);
        cout << tree.all_prod().value.val() << endl;
    }

    return 0;
}