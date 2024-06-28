#include <bits/stdc++.h>
using namespace std;

struct I {
    int l, r;
};

const int MAX_V = 200;
int V;
vector<int> g[MAX_V];
I A[MAX_V];

bool vis[MAX_V];
int T;
void dfs(int u) {
    vis[u] = true;

    for (int v : g[u]) {
        if (!vis[v]) {
            dfs(v);
        }
    }
}

bool movable(int i, int j) {
    if (A[j].l < A[i].r && A[i].r < A[j].r)
        return true;
    if (A[j].l < A[i].l && A[i].l < A[j].r)
        return true;
    return false;
}

int main() {
    int C; scanf("%d", &C);

    V = 0;
    while (C--) {
        int cmd, a, b;
        scanf("%d %d %d", &cmd, &a, &b);
        if (cmd == 1) {
            int idx = V++;
            A[idx] = (I) {a, b};

            for (int i = 0; i < idx; i++) {
                if (movable(i, idx)) {
                    g[i].push_back(idx);
                }
                if (movable(idx, i)) {
                    g[idx].push_back(i);
                }
            }
        }
        else {
            a--; b--;
            memset(vis, false, sizeof(vis));
            dfs(a);
            puts((vis[b]) ? "YES" : "NO");
        }
    }

    return 0;
}
