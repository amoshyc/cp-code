#include <bits/stdc++.h>
using namespace std;

const int MAX_V = 100000;

int V, E;
vector<int> g[MAX_V];

int vis[MAX_V]; // -1: not vis, 0: A, 1: Bs
bool possible = true;
vector<int> A, B;

void dfs(int v, int flag) {
    // printf("%d %d\n", v, flag);

    vis[v] = flag;

    if (!possible) return;
    if (g[v].size() == 0) return;

    if (flag == 0) {
        A.push_back(v);
    }
    else {
        B.push_back(v);
    }

    for (int u : g[v]) {
        if (vis[u] == -1) {
            dfs(u, 1 - flag);
        }
        else if (vis[u] == flag) {
            possible = false;
            return;
        }
    }
}

int main() {
    scanf("%d %d", &V, &E);
    for (int i = 0; i < E; i++) {
        int u, v; scanf("%d %d", &u, &v);
        u--; v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }

    memset(vis, -1, sizeof(vis));
    for (int v = 0; v < V; v++) {
        if (vis[v] == -1) {
            dfs(v, 0);
        }
    }

    if (!possible) puts("-1");
    else {
        printf("%d\n", int(A.size()));
        for (int v : A)
            printf("%d ", v + 1);
        puts("");

        printf("%d\n", int(B.size()));
        for (int v : B)
            printf("%d ", v + 1);
        puts("");
    }

    return 0;
}
