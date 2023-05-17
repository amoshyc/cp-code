#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint1000000007;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int H, W;
    cin >> H >> W;

    vector<string> S(H);
    for (int i = 0; i < H; i++) {
        cin >> S[i];
    }

    auto dp = vector<vector<mint>>(H, vector<mint>(W, 0));
    auto cntD = vector<mint>(H + W - 1, 0);
    auto cntT = vector<mint>(W, 0);

    dp[0][0] = 1;
    for (int r = 0; r < H; r++) {
        mint cntL = 0;
        for (int c = 0; c < W; c++) {

            if (r == 0 and c == 0) {
                cntL += dp[r][c];
                cntT[c] += dp[r][c];
                cntD[H - r - 1 + c] += dp[r][c];
                continue;
            }

            if (S[r][c] == '#') {
                cntL = 0;
                cntT[c] = 0;
                cntD[H - r - 1 + c] = 0;
                continue;
            }

            dp[r][c] = 0;
            if (c > 0) {
                dp[r][c] += cntL;
            }
            if (r > 0) {
                dp[r][c] += cntT[c];
            }
            if (r > 0 and c > 0) {
                dp[r][c] += cntD[H - r - 1 + c];
            }

            cntL += dp[r][c];
            cntT[c] += dp[r][c];
            cntD[H - r - 1 + c] += dp[r][c];
        }
    }

    cout << dp[H - 1][W - 1].val() << endl;

    return 0;
}