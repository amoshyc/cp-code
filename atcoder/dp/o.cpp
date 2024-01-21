#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
using ll = long long;
const ll INF = 1e18;
const ll M = 1e9 + 7;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    auto A = vector<vector<int>>(N, vector<int>(N));
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            cin >> A[i][j];
        }
    }

    // dp[i][S] = number of way to pair set S men with [0, i)
    // dp[i][S] is valid when len(S) = i
    // dp[i][S] -> dp[i + 1][S | j] for j not in S and A[i, j] = 1

    auto dp = vector<vector<ll>>(N + 1, vector<ll>(1 << N, 0));
    dp[0][0] = 1;
    for (int S = 0; S < (1 << N); S++) {
        int i = __builtin_popcount(S);
        for (int j = 0; j < N; j++) {
            if ((S & (1 << j)) == 0 and A[i][j] == 1) {
                dp[i + 1][S | (1 << j)] += dp[i][S];
                dp[i + 1][S | (1 << j)] %= M;
            }
        }
    }

    cout << dp[N][(1 << N) - 1] << endl;

    return 0;
}