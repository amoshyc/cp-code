#include <algorithm>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

struct HLD {
    vector<int> size;
    vector<int> depth;
    vector<int> heavy;
    vector<int> parent;
    vector<int> top;
    vector<int> tour;
    vector<int> t_enter;
    vector<int> t_leave;

    HLD(const vector<vector<int>> &adj, int root) {
        int N = adj.size();
        const int inf = 0x3f3f3f3f;

        // Find size, depth, heavy, parent by DFS
        {
            size = vector<int>(N, 1);
            depth = vector<int>(N, 0);
            heavy = vector<int>(N, inf);
            parent = vector<int>(N, inf);
            auto stack = vector<pair<char, int>>{{'l', root}, {'e', root}};
            depth[root] = 0;
            parent[root] = root;
            while (!stack.empty()) {
                auto [cmd, u] = stack.back();
                stack.pop_back();
                if (cmd == 'l') {
                    size[u] = 1;
                    heavy[u] = inf;
                    for (auto v : adj[u]) {
                        if (v != parent[u]) {
                            size[u] += size[v];
                            if (heavy[u] == inf || size[v] > size[heavy[u]]) {
                                heavy[u] = v;
                            }
                        }
                    }
                } else {
                    for (auto v : adj[u]) {
                        if (parent[v] == inf) {
                            parent[v] = u;
                            depth[v] = depth[u] + 1;
                            stack.push_back({'l', v});
                            stack.push_back({'e', v});
                        }
                    }
                }
            }
        }

        // Find top, tour, t_enter, t_leave by DFS
        {
            int time = 0;
            top = vector<int>(N, inf);
            tour = vector<int>(N, inf);
            t_enter = vector<int>(N, inf);
            t_leave = vector<int>(N, inf);
            auto stack = vector<pair<char, int>>{{'l', root}, {'e', root}};
            top[root] = root;
            while (!stack.empty()) {
                auto [cmd, u] = stack.back();
                stack.pop_back();
                if (cmd == 'l') {
                    t_leave[u] = time;
                } else {
                    t_enter[u] = time;
                    tour[time] = u;
                    time++;
                    for (auto v : adj[u]) {
                        if (v != parent[u]) {
                            // heavy edge / light edge
                            top[v] = ((v == heavy[u]) ? top[u] : v);
                            stack.push_back({'l', v});
                            stack.push_back({'e', v});
                        }
                    }
                }
            }
        }
    }

    int lca(int u, int v) {
        while (top[u] != top[v]) {
            if (depth[top[u]] > depth[top[v]]) {
                u = parent[top[u]];
            } else {
                v = parent[top[v]];
            }
        }
        return ((depth[u] < depth[v]) ? u : v);
    }
};

template <typename T> struct BIT {
    vector<T> data;

    BIT(int n) { data = vector<T>(n, 0); }

    void add(int idx, T val) {
        for (int i = idx + 1; i <= data.size() - 1; i += (i & (-i))) {
            data[i] = data[i] + val;
        }
    }

    T prefix(int idx) {
        T res = 0;
        for (int i = idx + 1; i > 0; i -= (i & -i)) {
            res = res + data[i];
        }
        return res;
    }

    T sum(int l, int r) { // l..r
        T val = prefix(r - 1);
        if (l != 0) {
            val -= prefix(l - 1);
        }
        return val;
    }
};

using ll = long long;

template <typename T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << ", ";
    }
    out << "]";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<vector<int>> adj(N);
    vector<tuple<int, int, ll>> edges;
    for (int i = 0; i < N - 1; i++) {
        int u, v;
        ll w;
        cin >> u >> v >> w;
        u--;
        v--;
        adj[u].push_back(v);
        adj[v].push_back(u);
        edges.push_back({u, v, w});
    }

    auto hld = HLD(adj, 0);

    auto bit = BIT<ll>(N + 1);
    for (auto [u, v, w] : edges) {
        if (hld.parent[u] == v) {
            swap(u, v);
        }
        bit.add(hld.t_enter[v], +w);
        bit.add(hld.t_leave[v], -w);
    }

    int Q;
    cin >> Q;
    while (Q--) {
        int cmd, a, b;
        cin >> cmd >> a >> b;

        if (cmd == 1) {
            auto [u, v, w] = edges[a - 1];
            if (hld.parent[u] == v) {
                swap(u, v);
            }
            ll nw = b;
            bit.add(hld.t_enter[v], nw - w);
            bit.add(hld.t_leave[v], -nw + w);
            edges[a - 1] = {u, v, nw};
        } else {
            a--;
            b--;
            int lca = hld.lca(a, b);
            ll da = bit.sum(0, hld.t_enter[a] + 1);
            ll db = bit.sum(0, hld.t_enter[b] + 1);
            ll dl = bit.sum(0, hld.t_enter[lca] + 1);
            ll ans = da + db - 2 * dl;
            cout << ans << "\n";
        }
    }

    return 0;
}