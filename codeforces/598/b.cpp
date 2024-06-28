#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s; cin >> s;
    int M; cin >> M;
    while (M--) {
        int l, r, k;
        cin >> l >> r >> k;
        l--; r--;
        k %= (r - l + 1);

        string sub_str = s.substr(r - k + 1, k);
        // cout << sub_str << ": ";
        for (int i = r; i - k >= l; i--) {
            s[i] = s[i - k];
        }
        for (int i = 0; i < k; i++)
            s[l + i] = sub_str[i];
        // cout << s << endl;
    }
    cout << s << "\n";

    return 0;
}
