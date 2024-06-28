#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 1e5;
const ll INF = 1e18;
int N;
ll w[MAX_N];
ll dp[MAX_N][2];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    for (int i = 0; i < N; i++) {
        cin >> w[i];
    }

    string p, rp, s, rs;
    cin >> p;
    rp = p;
    reverse(rp.begin(), rp.end());

    dp[0][0] = 0;
    dp[0][1] = w[0];

    for (int i = 1; i < N; i++) {
        cin >> s;
        rs = s;
        reverse(rs.begin(), rs.end());

        dp[i][0] = INF;
        if (s >= p) {
            dp[i][0] = min(dp[i][0], dp[i - 1][0]);
        }
        if (s >= rp) {
            dp[i][0] = min(dp[i][0], dp[i - 1][1]);
        }

        dp[i][1] = INF;
        if (rs >= p) {
            dp[i][1] = min(dp[i][1], dp[i - 1][0] + w[i]);
        }
        if (rs >= rp) {
            dp[i][1] = min(dp[i][1], dp[i - 1][1] + w[i]);
        }

        p = s;
        rp = rs;
    }
    //
    // for (int i = 0; i < N; i++) {
    //     cout << dp[i][0] << " " << dp[i][1] << endl;
    // }

    if (dp[N - 1][0] >= INF && dp[N - 1][1] >= INF) {
        cout << "-1" << endl;
    }
    else {
        cout << min(dp[N - 1][0], dp[N - 1][1]) << endl;
    }


    return 0;
}

// dp[i][0] = 使前 i + 1 個字串變成字典順序所需的最小成本，且第 i 個字串不轉
// dp[i][1] = 使前 i + 1 個字串變成字典順序所需的最小成本，且第 i 個字串反轉
