#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, T; cin >> N >> T;

    ll dp[N + 1][N + 1]; // store the numerators only
    memset(dp, 0, sizeof(dp));

    // see my previous python3 AC code for clearness
    dp[0][0] = T;
    for (int r = 0; r < N; r++) {
        for (int c = 0; c <= r; c++) {
            if (dp[r][c] >= (1 << r)) { // (1 << r) is denominator
                dp[r + 1][c + 0] += (dp[r][c] - (1 << r));
                dp[r + 1][c + 1] += (dp[r][c] - (1 << r));
            }
        }
    }

    int cnt = 0;
    for (int r = 0; r < N; r++) {
        for (int c = 0; c <= r; c++) {
            if (dp[r][c] >= (1 << r)) {
                cnt++;
            }
        }
    }

    cout << cnt << endl;

    return 0;
}
