#include <bits/stdc++.h>
using namespace std;

// 題目自己混用代號…

const int INF = 0x3f3f3f3f;

int N, M, R, C, A, B;

int solve(int tr, int tc) {
    int dr = abs(tr - R);
    int dc = abs(tc - C);

    if (dr % A != 0 || dc % B != 0) return INF;
    if ((dr / A) % 2 != (dc / B) % 2) return INF;
    return max(dr / A, dc / B);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> M >> R >> C >> A >> B;

    const int cr[4] = {1, N, 1, N};
    const int cc[4] = {1, 1, M, M};

    // already on corner
    for (int i = 0; i < 4; i++) {
        if (R == cr[i] && C == cc[i]) {
            cout << "0\n";
            return 0;
        }
    }

    // unable to move
    if ((R + A > N && R - A <= 0) || (C + B > M && C - B <= 0)) {
        cout << "Poor Inna and pony!\n";
        return 0;
    }

    int ans = INF;
    for (int i = 0; i < 4; i++) {
        ans = min(ans, solve(cr[i], cc[i]));
    }

    if (ans == INF)
        cout << "Poor Inna and pony!\n";
    else
        cout << ans << "\n";

    return 0;
}
