#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second

typedef long long ll;
typedef pair<ll, int> pii; // <d, v>
struct Edge {
    int to, w;
};

const int MAX_V = 100000;
const ll INF = 1e18;

int V, E, S; // V, Source
vector<Edge> g[MAX_V];
ll d[MAX_V];
int prevv[MAX_V];

void dijkstra() {
    fill(d, d + V, INF);
    fill(prevv, prevv + V, -1);
    priority_queue< pii, vector<pii>, greater<pii> > pq;

    d[S] = 0;
    pq.push(pii(0, S));

    while (!pq.empty()) {
        pii top = pq.top(); pq.pop();
        int u = top.nd;

        if (d[u] < top.st) continue;

        for (const Edge& e : g[u]) {
        // for (size_t i = 0; i < g[u].size(); i++) {
        //     const Edge& e = g[u][i];
            if (d[e.to] > d[u] + e.w) {
                d[e.to] = d[u] + e.w;
                prevv[e.to] = u;
                pq.push(pii(d[e.to], e.to));
            }
        }
    }
}

int main() {
    scanf("%d %d", &V, &E);
    for (int i = 0; i < E; i++) {
        int u, v, w; scanf("%d %d %d", &u, &v, &w);
        u--; v--;
        g[u].push_back((Edge) {v, w});
        g[v].push_back((Edge) {u, w});
    }

    S = 0;
    dijkstra();

    if (d[V - 1] == INF) {
        puts("-1");
        return 0;
    }

    vector<int> path;
    int v = V - 1;
    while (v != -1) {
        path.push_back(v);
        v = prevv[v];
    }

    reverse(path.begin(), path.end());
    for (int v : path) {
        printf("%d ", v + 1);
    }
    puts("");

    return 0;
}
