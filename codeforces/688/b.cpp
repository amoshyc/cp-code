#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s; cin >> s;
    string rs = s; reverse(rs.begin(), rs.end());
    cout << s << rs << "\n";

    return 0;
}
