#include <array>
#include <functional>
#include <iostream>
#include <map>
#include <sstream>
#include <vector>

using namespace std;
using i64 = long long;

template <typename T> int sz(const T &c) { return c.size(); }

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    auto arr = vector<vector<i64>>(3, vector<i64>(3, 0));
    for (int r = 0; r < 3; r++) {
        for (int c = 0; c < 3; c++) {
            cin >> arr[r][c];
        }
    }

    auto pos = vector<vector<int>>(3, vector<int>(3, -1));
    vector<i64> scores(2, 0);
    map<vector<vector<int>>, int> dp;

    auto check = [&](int r, int c, int dr, int dc) -> bool {
        bool ok = pos[r][c] != -1;
        for (int i = 0; i < 3; i++) {
            ok &= pos[r + i * dr][c + i * dc] == pos[r][c];
        }
        return ok;
    };

    function<int(int)> dfs = [&](int who) -> int {
        if (dp.contains(pos)) {
            return dp[pos];
        }

        // connect 3?
        int win = -1;
        for (int r = 0; r < 3; r++) {
            if (check(r, 0, 0, 1)) {
                win = pos[r][0];
            }
        }
        for (int c = 0; c < 3; c++) {
            if (check(0, c, 1, 0)) {
                win = pos[0][c];
            }
        }
        if (check(0, 0, 1, 1)) {
            win = pos[0][0];
        }
        if (check(2, 0, -1, 1)) {
            win = pos[2][0];
        }
        if (win != -1) {
            dp[pos] = win;
            return win;
        }

        vector<pair<int, int>> whites;
        for (int r = 0; r < 3; r++) {
            for (int c = 0; c < 3; c++) {
                if (pos[r][c] == -1) {
                    whites.push_back({r, c});
                }
            }
        }

        // game is over, compare scores
        if (sz(whites) == 0) {
            int win = scores[1] > scores[0];
            dp[pos] = win;
            return win;
        }

        // recursive
        bool any = false;
        for (auto [r, c] : whites) {
            scores[who] += arr[r][c];
            pos[r][c] = who;
            if (dfs(1 - who) == who) {
                any = true;
            }
            pos[r][c] = -1;
            scores[who] -= arr[r][c];
        }

        if (any) {
            dp[pos] = who;
            return who;
        } else {
            dp[pos] = 1 - who;
            return 1 - who;
        }
    };

    if (dfs(0) == 0) {
        cout << "Takahashi" << endl;
    } else {
        cout << "Aoki" << endl;
    }

    return 0;
}