#include <vector>
#include <iostream>
#include <algorithm>
using namespace std;

typedef long long ll;

ll solve() {
    int N, M;
    cin >> N >> M;

    vector<string> A(N);
    for (int i =0; i < N; i++) {
        cin >> A[i];
    }

    auto dp = vector<vector<ll>>(N, vector<ll>(M, 1ll));
    for (int r = 2; r < N; r++) {
        for (int c = 1; c < M; c++) {
            if (A[r][c] != A[r - 1][c - 1] or A[r][c] != A[r - 1][c] or 
                A[r][c] != A[r - 1][c + 1] or A[r][c] != A[r - 2][c]) {
                continue;
            }

            dp[r][c] = max(dp[r][c], 
                min({
                    dp[r - 1][c - 1], dp[r - 1][c], 
                    dp[r - 1][c + 1], dp[r - 2][c]
                }) + 1
            );

            // ll L1 = dp[r - 1][c - 1];
            // ll L2 = dp[r - 1][c];
            // ll L3 = dp[r - 1][c + 1];
            // ll L4 = dp[r - 2][c];

            // if (L1 <= min({L2, L3, L4})) {
            //     dp[r][c] = max(dp[r][c], L1 + 1);
            // }
            // if (L2 <= min({L1, L3, L4})) {
            //     dp[r][c] = max(dp[r][c], L2 + 1);
            // }
            // if (L3 <= min({L1, L2, L4})) {
            //     dp[r][c] = max(dp[r][c], L3 + 1);
            // }
            // if (L4 <= min({L1, L2, L3})) {
            //     dp[r][c] = max(dp[r][c], L4 + 1);
            // }
        }
    }

    ll ans = 0;
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            // cout << dp[r][c] << " ";
            ans += dp[r][c];
        }
        // cout << endl;
    }

    return ans;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout << solve() << endl;
    return 0;
}