#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

typedef long long ll;

template <class T>
ostream & operator << (ostream &out, const vector<T> v) {
    for (int i = 0; i < int(v.size()); i++) {
        out << v[i] << " ";
    }
    return out;
}

ll solve() {
    int R, C, K;
    cin >> R >> C >> K;

    auto A = vector<vector<ll>>(R, vector<ll>(C, 0ll));
    for (int k = 0; k < K; k++) {
        int r, c, v; cin >> r >> c >> v; r--; c--;
        A[r][c] = v;
    }

    // dp[r, c, p] = maximum sum of value from (0, 0) to (r, c) 
    //               if p items are picked in previous C grids
    auto dp = vector<vector<vector<ll>>>(R, vector<vector<ll>>(C, vector<ll>(4, 0)));
    dp[0][0][1] = ((A[0][0] > 0) ? A[0][0] : 0);
    for (int r = 0; r < R; r++) {
        for (int c = 0; c < C; c++) {
            // pick A[r][c]
            {
                // from left
                if (c >= 1) {
                    for (int p = 1; p < 4; p++) {
                        dp[r][c][p] = max(dp[r][c][p], dp[r][c - 1][p - 1] + A[r][c]);
                    }
                }
                // from top
                if (r >= 1) {
                    for (int p = 0; p < 4; p++) {
                        dp[r][c][1] = max(dp[r][c][1], dp[r - 1][c][p] + A[r][c]);
                    }
                }
            }
            // don't pick A[r][c]
            {
                // from left
                if (c >= 1) {
                    for (int p = 0; p < 4; p++) {
                        dp[r][c][p] = max(dp[r][c][p], dp[r][c - 1][p]);
                    }
                }
                // from top
                if (r >= 1) {
                    for (int p = 0; p < 4; p++) {
                        dp[r][c][0] = max(dp[r][c][0], dp[r - 1][c][p]);
                    }
                }
            }
        }
    }

    return *max_element(dp[R - 1][C - 1].begin(), dp[R - 1][C - 1].end());
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}