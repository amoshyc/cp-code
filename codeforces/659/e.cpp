#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
#define pb push_back

typedef pair<int, int> pii;

const int MAX_V = 100000;
const int MAX_E = 100000;
int V, E;
pii edge[MAX_E];

int par[MAX_V];
int vcnt[MAX_V];
int ecnt[MAX_V];

void init() {
    memset(par, -1, sizeof(par));
}

int root(int u) {
    if (par[u] < 0) return u;
    return (par[u] = root(par[u]));
}

void merge(int u, int v) {
    u = root(u);
    v = root(v);
    if (u == v) return;
    if (-par[u] > -par[v]) swap(u, v);
    par[v] += par[u];
    par[u] = v;
}

bool same(int u, int v) {
    return root(u) == root(v);
}

int main() {
    init();

    scanf("%d %d", &V, &E);
    for (int i = 0; i < E; i++) {
        int a, b; scanf("%d %d", &a, &b); a--; b--;
        edge[i] = pii(a, b);
        merge(a, b);
    }

    for (int i = 0; i < V; i++) {
        vcnt[root(i)]++;
    }
    for (int i = 0; i < E; i++) {
        ecnt[root(edge[i].st)]++;
    }

    int ans = 0;
    for (int i = 0; i < V; i++) {
        if (par[i] >= 0) continue;

        if (ecnt[i] == vcnt[i] - 1)
            ans++;
    }

    printf("%d\n", ans);

    return 0;
}
