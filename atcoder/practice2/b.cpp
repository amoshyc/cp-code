#include <iostream>
#include <utility>
#include <vector>
using namespace std;

using u64 = unsigned long long;

using S = u64;
using F = u64;

S default_data() { return 0; }

F default_lazy() { return 0; }

S op(S a, S b) { return a + b; }

S apply_lazy(F lazy, S data, int l, int r) { 
    return data + lazy * (r - l); 
}

F merge_lazy(F parent, F child) { 
    return parent + child; 
}

struct LazySegTree {
    int nn;
    vector<S> data;
    vector<F> lazy;

    LazySegTree(const vector<S> &arr) {
        const int n = arr.size();
        nn = 1;
        while (nn < n) {
            nn *= 2;
        }
        data = vector<S>(2 * nn, default_data());
        lazy = vector<F>(2 * nn, default_lazy());
        for (int i = 0; i < n; i++) {
            data[nn - 1 + i] = arr[i];
        }
        for (int u = nn - 2; u >= 0; u--) {
            data[u] = op(data[2 * u + 1], data[2 * u + 2]);
        }
    }

    void push(int u, int l, int r) {
        if (data[u] != default_data()) {
            int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
            data[lch] = apply_lazy(lazy[u], data[lch], l, m);
            data[rch] = apply_lazy(lazy[u], data[rch], m, r);
            lazy[lch] = merge_lazy(lazy[u], lazy[lch]);
            lazy[rch] = merge_lazy(lazy[u], lazy[rch]);
            lazy[u] = default_lazy();
        }
    }

    S query(int a, int b, int u, int l, int r) {
        if (l >= b || r <= a) {
            return default_data();
        }
        if (l >= a && r <= b) {
            return data[u];
        }
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        push(u, l, r);
        auto res1 = query(a, b, lch, l, m);
        auto res2 = query(a, b, rch, m, r);
        return op(res1, res2);
    }

    void modify(int a, int b, F x, int u, int l, int r) {
        if (l >= b || r <= a) {
            return;
        }
        if (l >= a && r <= b) {
            data[u] = apply_lazy(x, data[u], l, r);
            lazy[u] = merge_lazy(x, lazy[u]);
            return;
        }
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        push(u, l, r);
        modify(a, b, x, lch, l, m);
        modify(a, b, x, rch, m, r);
        data[u] = op(data[lch], data[rch]);
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, q;
    cin >> n >> q;

    vector<S> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }

    LazySegTree seg(arr);

    while (q--) {
        int cmd;
        cin >> cmd;
        if (cmd == 0) {
            int p, x;
            cin >> p >> x;
            seg.modify(p, p + 1, x, 0, 0, seg.nn);
        } else {
            int l, r;
            cin >> l >> r;
            cout << seg.query(l, r, 0, 0, seg.nn) << "\n";
        }
    }

    return 0;
}