#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint1000000007;
const int MAX_N = 1e5;

int N;
mint dp[MAX_N][2];
vector<int> adj[MAX_N];

mint f(int u, int c, int p) {
    if (dp[u][c] != 0) {
        return dp[u][c];
    }

    mint res = 1;
    for (auto v : adj[u]) {
        if (v != p) {
            if (c == 0) {
                res *= f(v, 1 - c, u);
            } else {
                res *= f(v, 0, u) + f(v, 1, u);
            }
        }
    }

    dp[u][c] = res;
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N;
    for (int i = 0; i < N - 1; i++) {
        int u, v;
        cin >> u >> v;
        u--;
        v--;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    for (int u = 0; u < N; u++) {
        dp[u][0] = dp[u][1] = 0;
    }
    mint ans = f(0, 0, -1) + f(0, 1, -1);
    cout << ans.val() << endl;

    return 0;
}