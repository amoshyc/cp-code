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

    auto A = vector<vector<ll>>(R + 1, vector<ll>(C + 1, 0));
    for (int k = 0; k < K; k++) {
        int r, c, v; cin >> r >> c >> v; r--; c--;
        A[r][c] = v;
    }

    auto dp = vector<vector<vector<ll>>>(R + 1, vector<vector<ll>>(C + 1, vector<ll>(5, 0)));
    dp[0][0][1] = ((A[0][0] > 0) ? A[0][0] : 0);
    
    for (int r = 0; r < R; r++) {
        for (int c = 0; c < C; c++) {
            for (int p = 0; p < 4; p++) {
                // walk down and pick
                if (A[r + 1][c] > 0) {
                    dp[r + 1][c][1] = max(dp[r + 1][c][1], dp[r][c][p] + A[r + 1][c]);
                }
                // walk down and don't pick
                dp[r + 1][c][0] = max(dp[r + 1][c][0], dp[r][c][p]);
                // walk right and pick
                if (A[r][c + 1] > 0) {
                    dp[r][c + 1][p + 1] = max(dp[r][c + 1][p + 1], dp[r][c][p] + A[r][c + 1]);
                }
                // walk right and don't pick
                dp[r][c + 1][p] = max(dp[r][c + 1][p], dp[r][c][p]);
            }
        }
    }

    return *max_element(dp[R - 1][C - 1].begin(), dp[R - 1][C - 1].begin() + 4);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}