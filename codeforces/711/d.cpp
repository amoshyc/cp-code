#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const ll mod = 1e9 + 7;
const int MAX_N = 200000;

int N;
int nxt[MAX_N];
int root[MAX_N];
int depth[MAX_N];
ll ans = 1;
int rest_cnt = 0;

ll fast_pow(ll a, ll b) {
    ll ans = 1;
    ll base = a;
    while (b) {
        if (b & 1)
            ans = ans * base % mod;
        base = base * base % mod;
        b >>= 1;
    }
    return ans;
}

void dfs(int u, int dep, int rt) {
    root[u] = rt;
    depth[u] = dep;

    int v = nxt[u];
    if (root[v] == -1) dfs(v, dep + 1, rt);
    else if (root[v] == root[u]) { // in same tree -> cycle
        int cycle_size = depth[u] - depth[v] + 1;
        ans = ans * ((fast_pow(2, cycle_size) - 2) + mod) % mod;
        rest_cnt -= cycle_size;
    }
}

int main() {
    memset(root, -1, sizeof(root));
    memset(depth, 0, sizeof(depth));

    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%d", &nxt[i]);
        nxt[i]--;
    }

    rest_cnt = N; // cnt of edges not in cycle
    for (int v = 0; v < N; v++) {
        if (root[v] == -1) {
            dfs(v, 0, v);
        }
    }

    ans = ans * fast_pow(2, rest_cnt) % mod;
    printf("%lld\n", ans);

    return 0;
}
