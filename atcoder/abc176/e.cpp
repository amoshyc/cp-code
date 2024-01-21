#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

// 0-based index
// tree size: 2 * NN - 1
// leaves indices: [NN - 1, 2 * NN - 1)
template<class T>
struct SegTree {
    int NN;
    T default_value;
    vector<T> data;

    T aggr(T a, T b) {
        return max(a, b);
    }

    SegTree(int N, T val) {
        default_value = val;
        NN = 1; while (NN < N) NN <<= 1;
        data = vector<T>(2 * NN - 1, val);
    }

    SegTree(const vector<T> v, T val) {
        default_value = val;
        int N = v.size();
        NN = 1; while (NN < N) NN <<= 1;
        data = vector<T>(2 * NN - 1, val);
        copy(v.begin(), v.end(), data.begin() + (NN - 1));
        build(0, 0, NN);
    }

    void build(int u, int l, int r) {
        if (r - l == 1) return;
        int m = (l + r) / 2;
        build(u * 2 + 1, l, m);
        build(u * 2 + 2, m, r);
        data[u] = aggr(data[2 * u + 1], data[2 * u + 2]);
    }

    T query(int a, int b, int u, int l, int r) {
        if (l >= b || r <= a)
            return default_value;
        if (l >= a && r <= b)
            return data[u];
        int m = (l + r) / 2;
        T res1 = query(a, b, u * 2 + 1, l, m);
        T res2 = query(a, b, u * 2 + 2, m, r);
        return aggr(res1, res2);
    }

    void update(int idx, T x, int u, int l, int r) {
        if (idx < l || idx >= r)
            return;
        if (idx == l && r - l == 1) {
            data[u] = x;
            return;
        }
        int m = (l + r) / 2;
        update(idx, x, u * 2 + 1, l, m);
        update(idx, x, u * 2 + 2, m, r);
        data[u] = aggr(data[2 * u + 1], data[2 * u + 2]);
    }

    void increment(int idx, T x, int u, int l, int r) {
        update(idx, data[NN - 1 + idx] + x, u, l, r);
    }
};

template <class T>
ostream & operator << (ostream &out, const SegTree<T> &seg) {
    out << "SegTree(NN=" << seg.NN << ", ";
    for (int u = seg.NN - 1, sz = seg.NN; sz >= 1; u /= 2, sz /= 2) {
        out << "[ ";
        for (int i = 0; i < sz; i++) {
            out << seg.data[u + i] << " ";
        }
        out << "]";
    }
    out << ")";
    return out;
}

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

    auto seg = SegTree<int>(cntC, 0);
    // cout << seg << endl;
    int ans = -1;
    for (int r = 0; r < H; r++) {
        for (auto c : t[r]) {
            seg.increment(c, -1, 0, 0, seg.NN);
        }
        // cout << seg << endl;
        int val = cntR[r] + seg.query(0, W, 0, 0, seg.NN);
        ans = max(ans, val);
        for (auto c : t[r]) {
            seg.increment(c, +1, 0, 0, seg.NN);
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