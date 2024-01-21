#include <algorithm>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint1000000007;

string K;
int N, D;
mint dp[10000 + 1][100][2];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> K >> D;
    N = K.size();

    reverse(K.begin(), K.end());

    dp[0][0][0] = 1;
    dp[0][0][1] = 1;
    for (int i = 0; i < N; i++) {
        for (int r = 0; r < D; r++) {
            int ki = K[i] - '0';
            // dp[i][r][0]
            for (int d = 0; d <= 9; d++) {
                dp[i + 1][(r + d) % D][0] += dp[i][r][0];
                if (d < ki) {
                    dp[i + 1][(r + d) % D][1] += dp[i][r][0];
                }
            }
            // dp[i][r][1]
            dp[i + 1][(r + ki) % D][1] += dp[i][r][1];
        }
    }

    cout << (dp[N][0][1] - 1).val() << endl;

    return 0;
}