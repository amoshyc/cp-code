#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int NM = 2000;
const ll MOD = 1e9 + 7;

int N, M;
string inp;
ll dp[NM + 1][NM + 1];

int main() {
    ios::sync_with_stdio(false); cin.tie(0);

    cin >> N >> M;
    cin >> inp;

    dp[0][0] = 1;
    for (int i = 1; i <= N - M; i++) {
        dp[i][0] = dp[i - 1][1];
        for (int j = 1; j <= N - M; j++) {
            dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j + 1]) % MOD;
        }
    }

    ll bs = 0, min_bs = 1e15;
    for (int i = 0; i < M; i++) {
        bs += ((inp[i] == '(') ? +1 : -1);
        min_bs = min(min_bs, bs);
    }

    ll ans = 0;
    for (int lp = 0; lp <= N - M; lp++) {
        for (int bp = 0; bp <= lp; bp++) {
            if (bp + min_bs < 0) continue;
            if (bp + bs > N - M) continue;

            ll lq = N - M - lp;
            ll bq = 0 - bp - bs;
            ans = (ans + dp[lp][bp] * dp[lq][-bq]) % MOD;
        }
    }

    cout << ans << "\n";

    return 0;
}
