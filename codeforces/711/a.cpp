#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    int N;
    cin >> N;

    vector<string> inp(N, "");
    for (int i = 0; i < N; i++) {
        cin >> inp[i];
    }

    bool ok = false;
    for (string& s: inp) {
        if (s[0] == 'O' && s[1] == 'O') {
            ok = true;
            s[0] = s[1] = '+';
            break;
        }
        if (s[3] == 'O' && s[4] == 'O') {
            ok = true;
            s[3] = s[4] = '+';
            break;
        }
    }

    cout << ((ok) ? "YES" : "NO") << endl;
    if (ok) {
        for (string i : inp) {
            cout << i << endl;
        }
    }

    return 0;
}
