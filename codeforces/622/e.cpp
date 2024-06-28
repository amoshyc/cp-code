#include <bits/stdc++.h>
using namespace std;

const int max_n = 500000;

int n;
int t[max_n];
int d[max_n];
vector<int> leaf; // children
vector<int> g[max_n];

void dfs(int u, int p, int dep) {
    d[u] = dep;

    if (g[u].size() == 1) {
        leaf.push_back(u);
        return;
    }

    for (int v : g[u]) {
        if (v != p) {
            dfs(v, u, dep + 1);
        }
    }
}

int main() {
    scanf("%d", &n);
    for (int i = 0; i < n - 1; i++) {
        int u, v; scanf("%d %d", &u, &v);
        u--; v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    int ans = -1;
    for (int u : g[0]) {
        leaf.clear();
        dfs(u, 0, 0);
        sort(leaf.begin(), leaf.end(), [&](int i, int j) {
            return d[i] < d[j];
        }); // 越淺的越早到

        t[0] = d[leaf[0]];
        ans = max(ans, t[0] + 1);
        for (int i = 1; i < leaf.size(); i++) {
            t[i] = max(d[leaf[i]], t[i - 1] + 1);
            ans = max(ans, t[i] + 1);
        }
    }

    printf("%d\n", ans);


    return 0;
}
