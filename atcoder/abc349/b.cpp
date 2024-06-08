#include <iostream>
#include <vector>

using namespace std;
using i64 = long long;

template <typename T> int sz(const T &c) { return c.size(); }

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    cin >> s;

    vector<int> cnt1(26, 0);
    for (char c : s) {
        cnt1[c - 'a'] += 1;
    }

    vector<int> cnt2(sz(s) + 1, 0);
    for (auto c : cnt1) {
        cnt2[c] += 1;
    }

    for (int i = 1; i <= sz(s); i++) {
        if (cnt2[i] != 0 && cnt2[i] != 2) {
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;

    return 0;
}