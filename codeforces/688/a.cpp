#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, D; cin >> N >> D;
    vector<int> a(D, 0);
    for (int i = 0; i < D; i++) {
        string inp; cin >> inp;

        if (find(inp.begin(), inp.end(), '0') != inp.end()) {
            a[i] = 1;
        }
        else {
            a[i] = 0;
        }
    }

    int len = 0;
    int ans = 0;
    for (int i = 0; i < D; i++) {
        if (a[i] == 1) {
            len++;
            ans = max(ans, len);
        }
        else {
            len = 0;
        }
    }

    cout << ans << "\n";

    return 0;
}
