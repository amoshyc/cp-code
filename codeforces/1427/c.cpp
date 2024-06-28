#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

const int INF = 0x3f3f3f3f;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int r, n;
    cin >> r >> n;
    auto t = vector<int>(n + 1);
    auto x = vector<int>(n + 1);
    auto y = vector<int>(n + 1);
    t[0] = 0;
    x[0] = 1;
    y[0] = 1;
    for (int i = 1; i <= n; i++) {
        cin >> t[i] >> x[i] >> y[i];
    }

    auto dp = vector<int>(n + 1, -INF);
    int pref = -INF;
    dp[0] = 0;
    for (int i = 1; i <= n; i++) {
        if (i >= 2 * r) {
            pref = max(pref, dp[i - 2 * r]);
        }
        dp[i] = pref + 1;
        for (int j = max(0, i - 2 * r); j < i; j++) {
            if (abs(x[i] - x[j]) + abs(y[i] - y[j]) <= t[i] - t[j]) {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }

    int ans = *max_element(dp.begin() + 1, dp.end());
    cout << max(ans, 0) << endl;

    return 0;
}