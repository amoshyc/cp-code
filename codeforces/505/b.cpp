#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
typedef pair<int, int> pii; // <e.to, color>

const int MAX_V = 100;
int V, E;
vector<pii> g[MAX_V];

int T, cnt;
bool vis[MAX_V][MAX_V];
void dfs(int u, int color) {
    vis[u][color] = true;

    if (u == T) {
        cnt++;
        return;
    }

    for (pii edge : g[u]) {
        if (!vis[edge.st][color] && edge.nd == color) {
            dfs(edge.st, color);
        }
    }
}

int main() {
    scanf("%d %d", &V, &E);
    for (int i = 0; i < E; i++) {
        int u, v, c; scanf("%d %d %d", &u, &v, &c); u--; v--;
        g[u].push_back(pii(v, c));
        g[v].push_back(pii(u, c));
    }

    int Q; scanf("%d", &Q);
    while (Q--) {
        int u, v; scanf("%d %d", &u, &v); u--; v--;

        cnt = 0;
        T = v;
        memset(vis, false, sizeof(vis));
        for (pii edge : g[u]) {
            dfs(u, edge.nd);
        }

        printf("%d\n", cnt);
    }

    return 0;
}
