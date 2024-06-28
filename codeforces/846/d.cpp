#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))
typedef long long ll;

struct Item {
    int r, c, t;
};

const int MAX_N = 500;
const int MAX_M = 500;
const int INF = 0x3f3f3f3f;
int N, M, K, Q;
int grid[MAX_N][MAX_M];
int S[MAX_N][MAX_M];
vector<Item> broken;

void init() {
    broken.clear();
}

void buildS() {
    S[0][0] = grid[0][0];
    for (int r = 1; r < N; r++) S[r][0] = grid[r][0] + S[r - 1][0];
    for (int c = 1; c < M; c++) S[0][c] = grid[0][c] + S[0][c - 1];
    for (int r = 1; r < N; r++) {
        for (int c = 1; c < M; c++) {
            S[r][c] = grid[r][c] + S[r - 1][c] + S[r][c - 1] - S[r - 1][c - 1];
        }
    }
}

int sum(int r1, int c1, int r2, int c2) { // [r1, r2) ~ [c1, c2)
    int sum = S[r2 - 1][c2 - 1];
    if (r1 > 0) sum -= S[r1 - 1][c2 - 1];
    if (c1 > 0) sum -= S[r2 - 1][c1 - 1];
    if (r1 > 0 && c1 > 0) sum += S[r1 - 1][c1 - 1];
    return sum;
}

bool C(int t) {
    memset(grid, 0, sizeof(grid));
    for (const auto& i : broken) {
        if (i.t <= t) {
            grid[i.r][i.c] = 1;        
        }
    }

    buildS();

    for (int r = 0; r + K - 1 < N; r++) {
        for (int c = 0; c + K - 1 < M; c++) {            
            if (sum(r, c, r + K, c + K) == K * K) {
                return true;
            }
        }
    }
    return false;
}

int solve() {
    int mn_time = INF, mx_time = -INF;
    for (auto x : broken) {
        mn_time = min(mn_time, x.t);
        mx_time = max(mx_time, x.t);
    }

    // C(t): is matrix broken at time t
    // 0 0 0 1 1 1
    int lb = mn_time, ub = mx_time;
    if (C(lb)) return lb;
    if (!C(ub)) return -1;
    while (ub - lb > 1) {
        int mid = (lb + ub) / 2;
        if (C(mid)) ub = mid;
        else lb = mid;
    }
    return ub;
}

int main() {
    while (scanf("%d %d %d %d", &N, &M, &K, &Q) != EOF) {
        init();
        for (int i = 0; i < Q; i++) {
            int r, c, t; scanf("%d %d %d", &r, &c, &t);
            broken.push_back(Item{r - 1, c - 1, t});
        }
        printf("%d\n", solve());
    }
    return 0;
}
