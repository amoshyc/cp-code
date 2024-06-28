#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
typedef pair<int, int> pii; // <e.to, w>

const int MAX_N = 2500;
int N;
int A[MAX_N][MAX_N];

struct Dsu {
    vector<int> par;

    Dsu(){}
    Dsu(int N) {
        par = vector<int>(N, -1);
    }

    int root(int a) {
        if (par[a] < 0) return a;
        return (par[a] = root(par[a]));
    }

    void unite(int a, int b) {
        a = root(a);
        b = root(b);
        if (a == b) return; // already in same set
        if (-par[a] > -par[b]) swap(a, b); // if (size[a] > size[b])
        par[b] += par[a]; // size[b] += size[a]
        par[a] = b;
    }

    bool same(int a, int b) {
        return root(a) == root(b);
    }

    int size(int a) { // correct if a is root
        return -par[a];
    }
};

struct Edge {
    int u, v, w;
    bool operator < (const Edge& e) const {
        return w < e.w;
    }
};

// kruskal
vector<Edge> edges;
vector<pii> mst[MAX_N];

// minimax distance
int B[MAX_N][MAX_N];

void kruskal() {
    for (int u = 0; u < N; u++)
        for (int v = u + 1; v < N; v++)
            edges.push_back((Edge) {u, v, A[u][v]});

    sort(edges.begin(), edges.end());

    Dsu dsu(N);

    for (const Edge& e : edges) {
        if (!dsu.same(e.u, e.v)) {
            dsu.unite(e.u, e.v);
            
            mst[e.u].push_back(pii(e.v, e.w));
            mst[e.v].push_back(pii(e.u, e.w));
        }
    }
}

void dfs(int s, int u, int p, int mx) {
    B[s][u] = mx;

    for (const pii& e : mst[u]) {
        int v = e.st;
        int w = e.nd;

        if (v == p) continue;
        else dfs(s, v, u, max(mx, w));
    }
}

bool solve() {
    for (int i = 0; i < N; i++)
        if (A[i][i] != 0)
            return false;

    for (int r = 0; r < N; r++)
        for (int c = 0; c < r; c++)
            if (A[r][c] != A[c][r])
                return false;

    kruskal();

    for (int u = 0; u < N; u++) {
        dfs(u, u, -1, 0);

        for (int v = 0; v < N; v++) {
            if (B[u][v] != A[u][v]) {
                return false;
            }
        }
    }

    return true;
}

int main() {
    scanf("%d", &N);
    for (int r = 0; r < N; r++)
        for (int c = 0; c < N; c++)
            scanf("%d", &A[r][c]);

    puts(((solve()) ? "MAGIC" : "NOT MAGIC"));

    return 0;
}
