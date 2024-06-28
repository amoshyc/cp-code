#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    string s;
    cin >> s;

    int cntI = 0, cntA = 0, cntF = 0;
    for (char c : s) {
        if (c == 'I') cntI++;
        if (c == 'A') cntA++;
        if (c == 'F') cntF++;
    }

    int ans = 0;
    for (char c : s) {
        if (c == 'I' && cntA + cntF == N - 1)
            ans++;
        if (c == 'A' && cntA - 1 + cntF== N - 1)
            ans++;
    }

    cout << ans << endl;

    return 0;
}
