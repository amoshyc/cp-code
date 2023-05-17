#include <iostream>
#include <vector>
#define sz(x) (int(x.size()))
using namespace std;

using ll = long long;

const int MAX_N = 100000;
ll N, mod;
ll dp[MAX_N];
ll ans[MAX_N];
vector<int> adj[MAX_N];

ll build(int u, int p) {
    dp[u] = 1;
    for (int v: adj[u]) {
        if (v != p) {
            dp[u] *= (1 + build(v, u));
            dp[u] %= mod;
        }
    }
    return dp[u];
}

void solve(int u, int p, ll top) {
    vector<ll> branches;
    for (int v : adj[u]) {
        if (v == p) {
            branches.push_back(1 + top);
        } else {
            branches.push_back(1 + dp[v]);
        }
    }

    if (sz(branches) == 0) {
        ans[u] = 1;
        return;
    }

    vector<ll> pref(sz(branches), 1);
    vector<ll> suff(sz(branches), 1);
    pref.front() = branches.front() % mod;
    for (int i = 1; i < sz(branches); i++) {
        pref[i] = (pref[i - 1] * branches[i]) % mod;
    }
    suff.back() = branches.back() % mod;
    for (int i = sz(branches) - 2; i >= 0; i--) {
        suff[i] = (suff[i + 1] * branches[i]) % mod;
    }

    ans[u] = pref.back();

    for (int i = 0; i < sz(adj[u]); i++) {
        int v = adj[u][i];
        if (v != p) {
            ll prod1 = ((i > 0) ? pref[i - 1]: 1);
            ll prod2 = ((i != sz(branches) - 1) ? suff[i + 1]: 1);
            solve(v, u, (prod1 * prod2) % mod);
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N >> mod;
    for (int i = 0; i < N - 1; i++) {
        int u, v;
        cin >> u >> v;
        u--; v--;
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    build(0, -1);
    solve(0, -1, 0);
    for (int i = 0; i < N; i++) {
        cout << ans[i] << "\n";
    }

    return 0;
}