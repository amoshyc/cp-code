#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>
using namespace std;

using real = long double;

int N;
real dp[301][301][301];

real E(int a, int b, int c) {
    // cout << a << " " << b << " " << c << endl;

    if (dp[a][b][c] >= -0.5) {
        return dp[a][b][c];
    }

    if (a == 0 && b == 0 && c == 0) {
        dp[a][b][c] = 0;
        return 0;
    }

    real res = 1;
    if (a >= 1) res += real(a) / N * E(a - 1, b, c);
    if (b >= 1) res += real(b) / N * E(a + 1, b - 1, c);
    if (c >= 1) res += real(c) / N * E(a, b + 1, c - 1);
    res /= real(a + b + c) / N;

    dp[a][b][c] = res;
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cin >> N;
    vector<int> cnt(4, 0);
    for (int i = 0; i < N; i++) {
        int a;
        cin >> a;
        cnt[a]++;
    }

    for (int i = 0; i <= 300; i++) {
        for (int j = 0; j <= 300; j++) {
            for (int k = 0; k <= 300; k++) {
                dp[i][j][k] = -1.0;
            }
        }
    }

    cout << fixed << setprecision(10) << E(cnt[1], cnt[2], cnt[3]) << endl;

    return 0;
}