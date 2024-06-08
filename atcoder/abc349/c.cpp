#include <cctype>
#include <iostream>
#include <vector>

using namespace std;
using i64 = long long;

template <typename T> int sz(const T &c) { return c.size(); }

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s, t;
    cin >> s >> t;

    vector<string> pats = {t};
    if (t[2] == 'X') {
        pats.push_back(t.substr(0, 2));
    }

    for (auto p : pats) {
        int j = 0;
        bool ok = true;
        for (char c : p) {
            while (j < sz(s) && toupper(s[j]) != c) {
                j += 1;
            }
            if (j < sz(s) && toupper(s[j]) == c) {
                j += 1;
            } else {
                ok = false;
                break;
            }
        }
        if (ok) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;

    return 0;
}