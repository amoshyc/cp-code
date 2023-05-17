#include <iostream>
#include <algorithm>
#include <vector>
#include <iomanip>
using namespace std;

using real = long double;

real dp[111][111][111];


real E(int a, int b, int c) {
    if (dp[a][b][c] >= 0) {
        return dp[a][b][c];
    }

    if (a >= 100 or b >= 100 or c >= 100) {
        dp[a][b][c] = 0;
        return dp[a][b][c];
    }

    real res = 1;
    res += real(a) / real(a + b + c) * E(a + 1, b, c);
    res += real(b) / real(a + b + c) * E(a, b + 1, c);
    res += real(c) / real(a + b + c) * E(a, b, c + 1);
    
    dp[a][b][c] = res;
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int a, b, c;
    cin >> a >> b >> c;

    for (int i = 0; i < 111; i++) {
        for (int j = 0; j < 111; j++) {
            for (int k = 0; k < 111; k++) {
                dp[i][j][k] = -1.0;
            }
        }
    }

    cout << fixed << setprecision(15) << E(a, b, c) << endl;

    return 0;
}