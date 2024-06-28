#include <bits/stdc++.h>
using namespace std;

int cnt[1 << 19];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int T;
    cin >> T;

    while (T--) {
        string cmd, s;
        cin >> cmd >> s;

        for (char& c : s) {
            c = (c - '0') % 2 + '0';
        }

        int id = bitset<18>(s).to_ulong();

        if (cmd[0] == '+') {
            cnt[id]++;
        }
        if (cmd[0] == '-') {
            cnt[id]--;
        }
        if (cmd[0] == '?') {
            printf("%d\n", cnt[id]);
        }
    }

    return 0;
}
