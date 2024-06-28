#include <bits/stdc++.h>
using namespace std;

struct DSU {
    int par[1000 * 1000];

    void init() {
        memset(par, -1, sizeof(par)); // par[i] = -rank[i] if i is root
    }

    int root(int a) {
        if (par[a] < 0) return a;
        return (par[a] = root(par[a]));
    }

    void unite(int a, int b) {
        a = root(a);
        b = root(b);
        if (a == b) return; // already in same set
        if (-par[a] > -par[b]) swap(a, b); // if (rank[a] > rank[b])
        par[b] += par[a]; // size[b] += size[a]
        par[a] = b;
    }

    bool same(int a, int b) {
        return root(a) == root(b);
    }

    int size(int a) {
        return -par[a];
    }
};

const int MAX_N = 1000;
int N, M;
char A[MAX_N][MAX_N + 1];
const int dr[4] = {0, 0, +1, -1};
const int dc[4] = {+1, -1, 0, 0};

DSU dsu;

inline int id(int r, int c) {
    return r * M + c;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int r = 0; r < N; r++) {
        scanf("%s", A[r]);
    }

    dsu.init();

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            if (A[r][c] == '.') {
                for (int i = 0; i < 4; i++) {
                    int nr = r + dr[i];
                    int nc = c + dc[i];
                    if (nr < 0 || nr >= N) continue;
                    if (nc < 0 || nc >= M) continue;
                    if (A[nr][nc] == '.') {
                        dsu.unite(id(r, c), id(nr, nc));
                    }
                }
            }
        }
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            if (A[r][c] == '.') printf(".");
            else {
                int cnt = 1; // self
                vector<int> adj;
                for (int i = 0; i < 4; i++) {
                    int nr = r + dr[i];
                    int nc = c + dc[i];
                    if (nr < 0 || nr >= N) continue;
                    if (nc < 0 || nc >= M) continue;
                    if (A[nr][nc] == '.') {
                        int root_id = dsu.root(id(nr, nc));
                        if (find(adj.begin(), adj.end(), root_id) == adj.end()) {
                            cnt += dsu.size(root_id);
                            adj.push_back(root_id);
                        }
                    }
                }

                printf("%d", cnt % 10);
            }
        }
        puts("");
    }


    return 0;
}
