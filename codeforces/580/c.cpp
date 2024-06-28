#include <bits/stdc++.h>
using namespace std;

int N, M;
const int MAX_N = 100000;
int cnt[MAX_N];
vector<int> G[MAX_N];
bool passed[MAX_N];
int has_cat[MAX_N];
int ans = 0;

void dfs(int v, int cats) {
    passed[v] = true;
    // printf("%d, %d\n", v, cats);

    if (cats + has_cat[v] == M + 1)
        return;

    if (v != 0 && cnt[v] == 1) {
        // printf("leaf : %d\n", v);
        ans++;
        return;
    }

    if (has_cat[v])
        cats++;
    else
        cats = 0;

    for (int c : G[v]) {
        if (!passed[c]) {
            dfs(c, cats);
        }
    }
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++)
        scanf("%d", &has_cat[i]);
    for (int i = 0; i < N - 1; i++) {
        int x, y; scanf("%d %d", &x, &y);
        x--; y--;
        cnt[x]++;
        cnt[y]++;
        G[x].push_back(y);
        G[y].push_back(x);
    }

    memset(passed, false, sizeof(passed));
    dfs(0, 0);

    printf("%d\n", ans);
    return 0;
}
