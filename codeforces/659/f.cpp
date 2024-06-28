#include <bits/stdc++.h>
using namespace std;

typedef tuple<int, int, int> tiii;
typedef long long ll;

const int dr[4] = {-1, 0, +1, 0};
const int dc[4] = {0, +1, 0, -1};

const int MAX_N = 1000;
const int MAX_M = 1000;
int N, M; ll K;
ll w[MAX_N][MAX_M];
vector<tiii> vs;

ll res[MAX_N][MAX_M];
ll cnt = 0, need, min_;

inline int id(int r, int c) {
    return r * M + c;
}

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
    par[b] += par[a]; // height[b] += height[a]
    par[a] = b;
}

bool same(int a, int b) {
    return root(a) == root(b);
}

void dfs(int r, int c) {
    if (cnt == need) return;

    cnt++;
    res[r][c] = min_;

    for (int i = 0; i < 4; i++) {
        int nr = r + dr[i];
        int nc = c + dc[i];
        if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
        if (res[nr][nc] != 0) continue;
        if (w[nr][nc] < min_) continue;

        dfs(nr, nc);
    }
}

bool solve() {
    sort(vs.begin(), vs.end(), greater<tiii>());

    init();
    for (auto v : vs) {
        int val = get<0>(v);
        int r = get<1>(v);
        int c = get<2>(v);
        for (int i = 0; i < 4; i++) {
            int nr = r + dr[i];
            int nc = c + dc[i];
            if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
            if (w[nr][nc] < w[r][c]) continue;
            if (!same(id(r, c), id(nr, nc)))
                unite(id(r, c), id(nr, nc));
        }

        int size = -par[root(id(r, c))];

        // cout << r << ", " << c << ": " << size << endl;

        if (K % val == 0) {
            need = K / val;
            if (need <= size) {
                min_ = val;
                cnt = 0;
                dfs(r, c);
                return true;
            }
        }
    }

    return false;
}

int main() {
    scanf("%d %d %lld", &N, &M, &K);

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            scanf("%lld", &w[i][j]);
            vs.push_back(tiii(w[i][j], i, j));
        }
    }

    if (solve()) {
        puts("YES");
        for (int r = 0; r < N; r++) {
            for (int c = 0; c < M; c++) {
                printf("%lld ", res[r][c]);
            }
            puts("");
        }
    }
    else {
        puts("NO");
    }


    return 0;
}
