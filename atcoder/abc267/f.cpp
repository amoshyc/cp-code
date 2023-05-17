#include <cmath>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;


template <typename T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << ", ";
    }
    out << "]";
    return out;
}

tuple<vector<int>, vector<int>, vector<int>> bfs(const vector<vector<int>> &G,
                                                 int root, int parent) {
    int N = G.size();
    auto vis = vector<int>();
    auto par = vector(N, -1);
    auto dep = vector(N, -1);
    auto que = queue<tuple<int, int>>();

    par[root] = parent;
    dep[root] = 0;
    que.push({root, parent});

    while (que.size() > 0) {
        auto [u, p] = que.front();
        que.pop();
        vis.push_back(u);
        for (auto v : G[u]) {
            if (v != p) {
                par[v] = u;
                dep[v] = dep[u] + 1;
                que.push({v, u});
            }
        }
    }

    return {vis, par, dep};
}

vector<vector<int>> build_dp(int N, vector<int> par) {
    const int L = floor(log2(N)) + 1;
    auto dp = vector(L, vector(N, -1));
    for (int u = 0; u < N; u++) {
        dp[0][u] = par[u];
    }
    for (int i = 1; i < L; i++) {
        for (int u = 0; u < N; u++) {
            if (dp[i - 1][u] != -1) {
                dp[i][u] = dp[i - 1][dp[i - 1][u]];
            }
        }
    }
    return dp;
}

int kth_parent(const vector<vector<int>> &dp, int u, int k) {
    const int L = dp.size();
    for (int i = 0; i < L; i++) {
        if (k & (1 << i)) {
            u = dp[i][u];
            if (u == -1) {
                return -1;
            }
        }
    }
    return u;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    auto G = vector(N, vector<int>());
    for (int i = 0; i < N - 1; i++) {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        G[u].push_back(v);
        G[v].push_back(u);
    }

    auto [vis, par, dep] = bfs(G, 0, -1);
    int s = vis.back();
    auto [s_vis, s_par, s_dep] = bfs(G, s, -1);
    int t = s_vis.back();
    auto [t_vis, t_par, t_dep] = bfs(G, t, -1);


    auto s_dp = build_dp(N, s_par);
    auto t_dp = build_dp(N, t_par);

    // cout << s_par << endl;
    // cout << t_par << endl;
    // cout << s_dp << endl;
    // cout << t_dp << endl;

    int Q;
    cin >> Q;
    for (int q = 0; q < Q; q++) {
        int u, k;
        cin >> u >> k;
        u--;

        int v1 = kth_parent(s_dp, u, k);
        if (v1 != -1) {
            cout << v1 + 1 << "\n";
            continue;
        }

        int v2 = kth_parent(t_dp, u, k);
        if (v2 != -1) {
            cout << v2 + 1 << "\n";
            continue;
        }

        cout << -1 << "\n";
    }

    return 0;
}