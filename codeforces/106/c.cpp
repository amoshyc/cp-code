#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 20;
const int MAX_W = 1000;
int N, W;
ll w[MAX_N + 1];
ll v[MAX_N + 1];
ll n[MAX_N + 1];
ll dp[MAX_N + 1][MAX_W + 1];

void knapsack() {
    for (int i = 1; i <= N; i++)
        for (int j = 0; j <= W; j++)
            for (int k = 0; k <= n[i] && j - k * w[i] >= 0; k++)
                dp[i][j] = max(dp[i][j], dp[i - 1][j - k * w[i]] + k * v[i]);
}

int main() {
    // ios::sync_with_stdio(false);
    // cin.tie(0);

    int NN, MM, c0, d0;
    scanf("%d %d %d %d", &NN, &MM, &c0, &d0);

    for (int i = 1; i <= MM; i++) {
        int a, b, c, d;
        scanf("%d %d %d %d", &a, &b, &c, &d);
        n[i] = a / b;
        w[i] = c;
        v[i] = d;
    }

    n[MM + 1] = NN / c0;
    w[MM + 1] = c0;
    v[MM + 1] = d0;

    N = MM + 1;
    W = NN;
    knapsack();

    printf("%lld\n", dp[N][W]);

    return 0;
}
