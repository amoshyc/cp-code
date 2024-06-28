#include <iostream>
#include <algorithm>
#include <vector>
#include <functional>
#include <cstdio>
#include <cstdlib>
#include <cstring>

using namespace std;
typedef pair<int, int> pii;
const int INF = 0x3f3f3f3f;

int N, M;
bool connect[4000][4000];
int deg[4000];
pii edges[4000];
// vector<int> g[4000];

int solve() {
    // int ans = INF;
    // for (int v1 = 0; v1 < N; v1++) {
    //     for (auto v2 : g[v1]) {
    //         for (auto v3 : g[v2]) {
    //             if (v3 == v1)
    //                 break;
    //             if (connect[v1][v3])
    //                 ans = min(ans, deg[v1] + deg[v2] + deg[v3] - 6);
    //         }
    //     }
    // }
    //
    // if (ans == INF)
    //     return -1;
    // return ans;
    int ans = INF;
    for (int i = 0; i < M; i++) {
        int v1 = edges[i].first;
        int v2 = edges[i].second;
        for (int v3 = 0; v3 < N; v3++) {
            if (v1 != v3 && v2 != v3 && connect[v1][v3] && connect[v2][v3])
                ans = min(ans, deg[v1] + deg[v2] + deg[v3] - 6);
        }
    }

    if (ans == INF)
        return -1;
    return ans;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < M; i++) {
        int a, b; scanf("%d %d", &a, &b);
        a--; b--;
        // g[a].push_back(b);
        // g[b].push_back(a);
        deg[a]++;
        deg[b]++;
        connect[a][b] = connect[b][a] = true;
        edges[i] = pii(a, b);
    }
    printf("%d\n", solve());

    return 0;
}
