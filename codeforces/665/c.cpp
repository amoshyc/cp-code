#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s;
    cin >> s;

    s += "$";

    for (size_t i = 1; i < s.length() - 1; i++) {
        if (s[i] == s[i - 1]) {
            if (s[i + 1] != 'z' && s[i - 1] != 'z') s[i] = 'z';
            else if (s[i + 1] != 'y' && s[i - 1] != 'y') s[i] = 'y';
            else s[i] = 'x';
        }
    }

    for (size_t i = 0; i < s.length() - 1; i++)
        cout << s[i];
    cout << "\n";

    return 0;
}
