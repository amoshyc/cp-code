#include <atcoder/modint>
#include <cstring>
#include <iostream>
#include <vector>
using namespace std;

using ll = long long;

int N, D;
ll M = 1e9 + 7;
string K;
ll dp[10000 + 1][100][2];

ll f(int i, int rem, bool is_pref) {
    if (i >= N) {
        return int(rem == 0);
    }

    if (dp[i][rem][is_pref] != -1) {
        return dp[i][rem][is_pref];
    }

    ll res = 0;
    for (int d = 0; d < ((is_pref) ? K[i] - '0' + 1 : 10); d++) {
        res += f(i + 1, (rem + d) % D, (is_pref && d == K[i] - '0'));
        res %= M;
    }
    dp[i][rem][is_pref] = res;

    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    memset(dp, -1, sizeof(dp));

    cin >> K;
    cin >> D;
    N = K.size();
    cout << (f(0, 0, true) - 1 + M) % M << endl;

    return 0;
}