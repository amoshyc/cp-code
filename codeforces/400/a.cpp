#include <bits/stdc++.h>
using namespace std;

vector<string> ans;

void solve(string s) {
    ans.clear();

    // 1x12
    if (any_of(s.begin(), s.end(), [](const char& c) { return c == 'X'; }))
        ans.push_back("1x12");

    // 2x6
    for (int i = 0; i < 6; i++) {
        if (s[i] == 'X' && s[i + 6] == 'X') {
            ans.push_back("2x6");
            break;
        }
    }

    // 3x4
    for (int i = 0; i < 4; i++) {
        if (s[i] == 'X' && s[i + 4] == 'X' && s[i + 8] == 'X') {
            ans.push_back("3x4");
            break;
        }
    }

    // 4x3
    for (int i = 0; i < 3; i++) {
        if (s[i + 0] == 'X' && s[i + 3] == 'X' &&
            s[i + 6] == 'X' && s[i + 9] == 'X') {
            ans.push_back("4x3");
            break;
        }
    }

    // 6x2
    for (int i = 0; i < 2; i++) {
        if (s[i + 0] == 'X' && s[i + 2] == 'X' && s[i + 4] == 'X' &&
            s[i + 6] == 'X' && s[i + 8] == 'X' && s[i + 10] == 'X') {
            ans.push_back("6x2");
            break;
        }
    }

    // 12x1
    if (all_of(s.begin(), s.end(), [](const char& c) { return c == 'X'; }))
        ans.push_back("12x1");

}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int T;
    cin >> T;

    while (T--) {
        string inp; cin >> inp;

        solve(inp);

        int size = ans.size();
        cout << size;
        for (int i = 0; i < size; i++) {
            cout << " " << ans[i];
        }
        cout << "\n";
    }

    return 0;
}
