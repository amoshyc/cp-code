#include <bits/stdc++.h>
using namespace std;

template <class T>
struct SegTree {
    int NN;            // tree size: 2 * NN - 1
    T dflt;            // default val
    vector<T> seg;     // 0-based index

    int init(int n, T val) {
        dflt = val;
        NN = 1;
        while (NN < n)
            NN <<= 1;
        seg.clear();
        seg.resize(2 * NN, dflt);
        return NN;
    }

    T func(T a, T b) {
        return max(a, b);
    }

    void gather(int u, int l, int r) {
        seg[u] = func(seg[u * 2 + 1], seg[u * 2 + 2]);
    }

    void build(int u, int l, int r) {
        if (r - l == 1)
            return;
        int m = (l + r) / 2;
        build(u * 2 + 1, l, m);
        build(u * 2 + 2, m, r);
        gather(u, l, r);
    }

    T query(int a, int b, int u, int l, int r) {
        if (l >= b || r <= a)
            return dflt;
        if (l >= a && r <= b)
            return seg[u];
        int m = (l + r) / 2;
        T res1 = query(a, b, u * 2 + 1, l, m);
        T res2 = query(a, b, u * 2 + 2, m, r);
        return func(res1, res2);
    }

    void update(int idx, T x, int u, int l, int r) {
        if (idx < l || idx >= r)
            return;
        if (idx == l && r - l == 1) {
            seg[u] += x;
            return;
        }
        int m = (l + r) / 2;
        update(idx, x, u * 2 + 1, l, m);
        update(idx, x, u * 2 + 2, m, r);
        gather(u, l, r);
    }
};

int solve() {
    int H, W, M;
    cin >> H >> W >> M;

    auto cntR = vector<int>(H, 0);
    auto cntC = vector<int>(W, 0);
    auto t = vector<vector<int>>(H, vector<int>());
    for (int i = 0; i < M; i++) {
        int r, c; 
        cin >> r >> c;
        r--; c--;
        cntR[r]++;
        cntC[c]++;
        t[r].push_back(c);
    }

    auto seg = SegTree<int>();
    int NN = seg.init(W, 0);
    copy(cntC.begin(), cntC.end(), seg.seg.begin() + NN - 1);
    seg.build(0, 0, NN);

    int ans = -1;
    for (int r = 0; r < H; r++) {
        for (auto c : t[r]) {
            seg.update(c, -1, 0, 0, NN);
        }

        int val = cntR[r] + seg.query(0, W, 0, 0, NN);
        ans = max(ans, val);
        for (auto c : t[r]) {
            seg.update(c, +1, 0, 0, NN);
        }
    }
    return ans;
}

int main () {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}