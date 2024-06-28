#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    string s1, s2;
    cin >> N;
    cin >> s1 >> s2;

    // 11100000
    // 10011110
    // is Draw

    int cnt[4] = {0};
    for (int i = 0; i < 2 * N; i++) {
        int flag = 0;
        if (s1[i] == '1') flag += (1 << 0);
        if (s2[i] == '1') flag += (1 << 1);
        cnt[flag]++;
    }

    int res[2] = {0};
    int choice[2][4] = { // 優先打擊對手
        {3, 2, 1, 0},
        {3, 1, 2, 0}
    };

    for (int r = 0; r < N; r++) { // round
        for (int i = 0; i < 2; i++) { // player
            for (int ch : choice[i]) {
                if (cnt[ch] > 0) {
                    // cout << i << " get " << ch << endl;
                    cnt[ch]--;
                    if (ch & (1 << i))
                        res[i]++;
                    break;
                }
            }
        }
    }

    // cout << res[0] << endl;
    // cout << res[1] << endl;

    if (res[0] == res[1]) cout << "Draw" << endl;
    else {
        if (res[0] > res[1])
            cout << "First" << endl;
        else
            cout << "Second" << endl;
    }

    return 0;
}
