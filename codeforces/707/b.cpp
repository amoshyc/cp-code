#include <bits/stdc++.h>
using namespace std;

const int INF = 0x3f3f3f3f;

struct Edge {
    int to, w;
};

bool isStorage[100000];
vector<Edge> g[100000];

int main() {
    int N, M, K;
    scanf("%d %d %d", &N, &M, &K);

    for (int i = 0; i < M; i++) {
        int u, v, w;
        scanf("%d %d %d", &u, &v, &w);
        u--; v--;

        g[u].push_back((Edge) {v, w});
        g[v].push_back((Edge) {u, w});
    }

    for (int i = 0; i < K; i++) {
        int v; scanf("%d", &v); v--;
        isStorage[v] = true;
    }

    int ans = INF;
    for (int v = 0; v < N; v++) {
        if (isStorage[v]) {
            for (auto e : g[v]) {
                if (!isStorage[e.to]) {
                    ans = min(ans, e.w);
                }
            }
        }
    }

    if (ans == INF) puts("-1");
    else {
        printf("%d\n", ans);
    }

    return 0;
}
