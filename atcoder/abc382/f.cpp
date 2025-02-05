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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int h, w, n;
    cin >> h >> w >> n;
    vector<tuple<int, int, int, int>> bars;
    for (int i = 0; i < n; i++) {
        int r, c, l;
        cin >> r >> c >> l;
        r--;
        c--;
        bars.push_back({r, c, l, i});
    }
    sort(bars.begin(), bars.end());
    reverse(bars.begin(), bars.end());

    vector<int> ans(n, 0);
    vector<int> arr(w, h);

    using S = int;
    using F = int;
    constexpr S default_data = 1'000'000;
    constexpr F default_lazy = 1'000'000;
    auto op = [](S a, S b) -> S { return min(a, b); };
    auto apply_lazy = [](F lazy, S, int, int) -> S { return lazy; };
    auto merge_lazy = [](F parent, F) -> F { return parent; };
    auto seg = LazySegTree<S, F, default_data, default_lazy, op, apply_lazy,
                           merge_lazy>(arr);

    for (auto [r, c, l, i] : bars) {
        int bottom = seg.query(c, c + l, 0, 0, seg.nn);
        ans[i] = bottom - 1;
        seg.modify(c, c + l, ans[i], 0, 0, seg.nn);
    }

    for (auto &x : ans) {
        cout << x + 1 << "\n";
    }

    return 0;
}