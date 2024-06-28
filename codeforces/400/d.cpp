#include <bits/stdc++.h>
using namespace std;

const int inf = 0x3f3f3f3f;
const int max_n = 100000;
const int max_k = 500;
int N, M, K;
int num[max_k];
int type[max_n];

int parent[max_n];
int treesz[max_n];

int d[max_k][max_k];

void init() {
    // dsu_init
    for (int i = 0; i < N; i++) {
        parent[i] = i;
        treesz[i] = 1;
    }

    // type init
    int idx = 0;
    for (int i = 0; i < K; i++) {
        fill(type + idx, type + idx + num[i], i);
        idx += num[i];
    }

    // d init
    memset(d, inf, sizeof(d));
    for (int i = 0; i < K; i++)
        d[i][i] = 0;
}

int dsu_find(int a) {
    if (parent[a] == a)
        return a;
    return parent[a] = dsu_find(parent[a]);
}

void dsu_unite(int a, int b) {
    a = dsu_find(a);
    b = dsu_find(b);
    if (a == b) return;

    if (treesz[a] < treesz[b]) {
        parent[a] = b;
        treesz[b] += treesz[a];
    }
    else {
        parent[b] = a;
        treesz[a] += treesz[b];
    }
}

bool dsu_same(int a, int b) {
    return dsu_find(a) == dsu_find(b);
}

bool solve() {
    int idx = 0;
    for (int i = 0; i < K; i++) {
        for (int j = 0; j < num[i]; j++)
            if (!dsu_same(idx, idx + j))
                return false;
        idx += num[i];
    }

    for (int k = 0; k < K; k++)
        for (int i = 0; i < K; i++)
            for (int j = 0; j < K; j++)
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);

    return true;
}

int main() {
    scanf("%d %d %d", &N, &M, &K);
    for (int i = 0; i < K; i++)
        scanf("%d", &num[i]);

    init();

    while (M--) {
        int u, v, w;
        scanf("%d %d %d", &u, &v, &w);
        u--; v--;

        d[type[u]][type[v]] = min(d[type[u]][type[v]], w);
        d[type[v]][type[u]] = min(d[type[v]][type[u]], w);
        
        if (w == 0)
            dsu_unite(u, v);
    }

    if (!solve()) {
        puts("No");
    }
    else {
        puts("Yes");
        for (int i = 0; i < K; i++) {
            for (int j = 0; j < K; j++) {
                printf("%d ", (d[i][j] == inf) ? -1 : d[i][j]);
            }
            puts("");
        }
    }

    return 0;
}
