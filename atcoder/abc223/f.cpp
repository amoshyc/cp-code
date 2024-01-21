#include <algorithm>
#include <cstdio>
#include <functional>
#include <iostream>
#include <vector>
using namespace std;

template <typename S, typename F> struct LazySegTree {
    int nn;
    vector<S> data;
    vector<F> lazy;

    S default_data;
    F default_lazy;
    function<S(S, S)> op;
    function<S(F, S, int, int, int)> apply_lazy;
    function<F(F, F)> merge_lazy;

    LazySegTree(int n, S default_data, F default_lazy, auto &op,
                auto &apply_lazy, auto &merge_lazy) {
        this->nn = 1;
        while (this->nn < n) {
            this->nn <<= 1;
        }
        this->data = vector<S>(2 * this->nn, default_data);
        this->lazy = vector<F>(2 * this->nn, default_lazy);
        this->default_data = default_data;
        this->default_lazy = default_lazy;
        this->op = op;
        this->apply_lazy = apply_lazy;
        this->merge_lazy = merge_lazy;
    }

    void init(const vector<S> &arr) {
        copy(arr.begin(), arr.end(), data.begin() + (nn - 1));
        for (int i = nn - 2; i >= 0; i--) {
            data[i] = op(data[2 * i + 1], data[2 * i + 2]);
        }
    }

    void push(int u, int l, int r) {
        if (lazy[u] == default_lazy) {
            return;
        }
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        data[lch] = apply_lazy(lazy[u], data[lch], lch, l, m);
        data[rch] = apply_lazy(lazy[u], data[rch], rch, m, r);
        lazy[lch] = merge_lazy(lazy[u], lazy[lch]);
        lazy[rch] = merge_lazy(lazy[u], lazy[rch]);
        lazy[u] = default_lazy;
    }

    S prod(int a, int b, int u, int l, int r) {
        if (l >= b || r <= a)
            return default_data;
        if (l >= a && r <= b)
            return data[u];
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        push(u, l, r);
        S res1 = prod(a, b, lch, l, m);
        S res2 = prod(a, b, rch, m, r);
        return op(res1, res2);
    }

    void apply(int a, int b, F x, int u, int l, int r) {
        if (l >= b || r <= a)
            return;
        if (l >= a && r <= b) {
            data[u] = apply_lazy(x, data[u], u, l, r);
            lazy[u] = merge_lazy(x, lazy[u]);
            return;
        }
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        push(u, l, r);
        apply(a, b, x, lch, l, m);
        apply(a, b, x, rch, m, r);
        data[u] = op(data[lch], data[rch]);
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, Q;
    cin >> N >> Q;
    string s;
    cin >> s;

    vector<int> P(N, 0);
    P[0] = ((s[0] == '(') ? +1 : -1);
    for (int i = 1; i < N; i++) {
        P[i] = P[i - 1] + ((s[i] == '(') ? +1 : -1);
    }

    using S = int;
    using F = int;
    S default_data = 0x3f3f3f3f;
    F default_lazy = 0;
    auto op = [](S a, S b) -> S { return min(a, b); };
    auto apply_lazy = [](F lazy, S data, int u, int l, int r) -> S {
        return lazy + data;
    };
    auto merge_lazy = [](F parent, F child) -> F { return parent + child; };
    LazySegTree seg(N, default_data, default_lazy, op, apply_lazy, merge_lazy);
    seg.init(P);

    while (Q--) {
        int cmd, l, r;
        cin >> cmd >> l >> r;
        l--;
        r--;
        if (cmd == 1) {
            if (s[l] == '(' and s[r] == ')') {
                seg.apply(l, r, -2, 0, 0, seg.nn);
                swap(s[l], s[r]);
            } else if (s[l] == ')' and s[r] == '(') {
                seg.apply(l, r, +2, 0, 0, seg.nn);
                swap(s[l], s[r]);
            }
        } else { // closed interval [l, r]
            // endpoint check
            bool check1 = s[l] == '(' and s[r] == ')';
            // bracket balance
            bool check2 = seg.prod(r, r + 1, 0, 0, seg.nn) ==
                          ((l > 0) ? seg.prod(l - 1, l, 0, 0, seg.nn) : 0);
            // cannot drop under P[l] - 1
            bool check3 = seg.prod(l, r + 1, 0, 0, seg.nn) ==
                          seg.prod(l, l + 1, 0, 0, seg.nn) - 1;

            if (check1 and check2 and check3) {
                cout << "Yes\n";
            } else {
                cout << "No\n";
            }
        }
    }

    return 0;
}