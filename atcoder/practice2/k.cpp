#include <algorithm>
#include <functional>
#include <iostream>
#include <queue>
#include <sstream>
#include <utility>
#include <vector>
using namespace std;

template <typename S, typename F, S default_data, F default_lazy, auto op,
          auto apply_lazy, auto merge_lazy>
struct LazySegTree {
    int nn;
    vector<S> data;
    vector<F> lazy;

    LazySegTree(const vector<S> &arr) {
        nn = 1;
        while (nn < int(arr.size())) {
            nn <<= 1;
        }
        data.assign(2 * nn, default_data);
        lazy.assign(2 * nn, default_lazy);
        for (int i = 0; i < int(arr.size()); i++) {
            data[nn - 1 + i] = arr[i];
        }
        for (int u = nn - 2; u >= 0; u--) {
            data[u] = op(data[2 * u + 1], data[2 * u + 2]);
        }
    }

    void push(int u, int l, int r) {
        if (lazy[u] != default_lazy) {
            int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
            data[lch] = apply_lazy(lazy[u], data[lch], l, m);
            data[rch] = apply_lazy(lazy[u], data[rch], m, r);
            lazy[lch] = merge_lazy(lazy[u], lazy[lch]);
            lazy[rch] = merge_lazy(lazy[u], lazy[rch]);
            lazy[u] = default_lazy;
        }
    }

    S query(int a, int b, int u, int l, int r) {
        if (l >= b || r <= a) {
            return default_data;
        }
        if (a <= l && r <= b) {
            return data[u];
        }
        push(u, l, r);
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        return op(query(a, b, lch, l, m), query(a, b, rch, m, r));
    }

    void modify(int a, int b, F x, int u, int l, int r) {
        if (l >= b || r <= a) {
            return;
        }
        if (a <= l && r <= b) {
            data[u] = apply_lazy(x, data[u], l, r);
            lazy[u] = merge_lazy(x, lazy[u]);
            return;
        }
        push(u, l, r);
        int m = (l + r) / 2, lch = 2 * u + 1, rch = 2 * u + 2;
        modify(a, b, x, lch, l, m);
        modify(a, b, x, rch, m, r);
        data[u] = op(data[lch], data[rch]);
    }
};

const uint64_t M = 998'244'353;

using S = uint64_t;
using F = pair<uint64_t, uint64_t>;
constexpr S default_data = 0;
constexpr F default_lazy = {1, 0};

S op(S a, S b) { return (a + b) % M; }

S apply_lazy(F lazy, S data, int l, int r) {
    auto [b, c] = lazy;
    auto res = b * data % M;
    res += c * uint64_t(r - l);
    return res % M;
}
F merge_lazy(F parent, F child) {
    auto [b1, c1] = parent;
    auto [b2, c2] = child;
    auto b = b1 * b2 % M;
    auto c = (b1 * c2 % M + c1) % M;
    return {b, c};
}
using SegTree =
    LazySegTree<S, F, default_data, default_lazy, op, apply_lazy, merge_lazy>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, q;
    cin >> n >> q;
    vector<uint64_t> arr(n);
    for (auto &x : arr) {
        cin >> x;
    }

    SegTree seg(arr);

    vector<int> ans;
    while (q--) {
        int cmd;
        cin >> cmd;
        if (cmd == 0) {
            int l, r;
            uint64_t b, c;
            cin >> l >> r >> b >> c;
            seg.modify(l, r, {b, c}, 0, 0, seg.nn);
        } else {
            int l, r;
            cin >> l >> r;
            cout << seg.query(l, r, 0, 0, seg.nn) << "\n";
        }
    }

    return 0;
}