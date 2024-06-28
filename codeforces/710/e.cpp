#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 1e7;
const ll INF = 1e18;

int N; ll x, y;
ll dp[MAX_N + 1];

int main() {
    cin >> N >> x >> y;

    fill(dp + 1, dp + N + 1, INF);

    // base case
    dp[1] = x;
    for (int i = 2; i <= N; i++) {
        dp[i] = min(dp[i], dp[i - 1] + x);
        if (i & 1)
            dp[i] = min(dp[i], dp[(i + 1) / 2] + y + x);
        else
            dp[i] = min(dp[i], dp[i / 2] + y);
    }

    cout << dp[N] << endl;

    return 0;
}
