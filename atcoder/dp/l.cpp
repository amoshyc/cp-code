#include <iostream>
#include <tuple>
#include <unordered_map>
#include <vector>
using namespace std;

using ll = long long;
const ll INF = 1e18;

int N;
ll A[3000];
ll dp[3000][3000][2];

ll f(int i, int j, bool is_tora) {
    if (dp[i][j][is_tora] != INF) {
        return dp[i][j][is_tora];
    }

    ll res = 0;
    if (i == j) {
        res = (is_tora) ? A[i] : -A[i];
    } else if (is_tora) {
        res = max(f(i + 1, j, false) + A[i], f(i, j - 1, false) + A[j]);
    } else {
        res = min(f(i + 1, j, true) - A[i], f(i, j - 1, true) - A[j]);
    }
    
    dp[i][j][is_tora] = res;

    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N;
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            dp[i][j][0] = INF;
            dp[i][j][1] = INF;
        }
    }
    cout << f(0, N - 1, true) << endl;

    return 0;
}