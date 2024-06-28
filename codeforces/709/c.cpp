#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s;
    cin >> s;

    auto st = find_if(s.begin(), s.end(), [](char c) { return c != 'a'; });
    if (st == s.end()) {
        s.back() = 'z';
        cout << s << endl;
        return 0;
    }

    auto ed = find_if(st, s.end(), [](char c) { return c == 'a'; });
    for (auto i = st; i != ed; i++)
        *i = char(*i - 1);
    cout << s << endl;

    return 0;
}
