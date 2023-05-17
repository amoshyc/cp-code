#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;
using ll = long long;
const ll INF = 1e18;

int N;
ll A[400];
ll P[400];
ll dp[400][400 + 1];

ll sum(int i, int j) { return P[j - 1] - ((i >= 1) ? P[i - 1] : 0); }

ll f(int i, int j) { // [i, j)
    if (dp[i][j] != INF) {
        return dp[i][j];
    }

    if (j - i == 1) {
        return 0;
    }

    ll ret = INF;
    for (int k = i + 1; k < j; k++) {
        ll val = f(i, k) + f(k, j) + sum(i, k) + sum(k, j);
        ret = min(ret, val);
    }

    dp[i][j] = ret;

    return ret;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N;
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    P[0] = A[0];
    for (int i = 1; i < N; i++) {
        P[i] = P[i - 1] + A[i];
    }

    for (int i = 0; i < N; i++) {
        for (int j = i; j <= N; j++) {
            dp[i][j] = INF;
        }
    }

    cout << f(0, N) << endl;

    return 0;
}