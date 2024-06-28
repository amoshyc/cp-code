#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int dr[8] = {-1, 0, +1, +1, +1, 0, -1, -1};
int dc[8] = {-1, -1, -1, 0, +1, +1, +1, 0};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s; cin >> s;

    int c = s[0] - 'a';
    int r = s[1] - '1';

    // cout << r << " " << c << endl;

    int cnt = 0;
    for (int i = 0; i < 8; i++) {
        int nr = r + dr[i];
        int nc = c + dc[i];

        // cout << nr << " " << nc << endl;

        if (nr < 0 || nr >= 8 || nc < 0 || nc >= 8) continue;
        cnt++;
    }

    cout << cnt << endl;

    return 0;
}
