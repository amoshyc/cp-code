#include <bits/stdc++.h>
using namespace std;

int N;
string s;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> s;
    N = int(s.length());

    char mp[4];
    for (int i = 0; i < N; i++) {
        if (s[i] == 'R' || s[i] == 'B' || s[i] == 'Y' || s[i] == 'G') {
            mp[i % 4] = s[i];
        }
    }

    int cnt[4] = {0};
    for (int i = 0; i < N; i++) {
        if (s[i] == '!') {
            s[i] = mp[i % 4];
            if (mp[i % 4] == 'R') cnt[0]++;
            if (mp[i % 4] == 'B') cnt[1]++;
            if (mp[i % 4] == 'Y') cnt[2]++;
            if (mp[i % 4] == 'G') cnt[3]++;
        }
    }

    for (int i = 0; i < 4; i++) {
        cout << cnt[i] << " ";
    }
    cout << endl;


    return 0;
}
