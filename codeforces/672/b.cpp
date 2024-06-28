#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; cin >> N;
    string s; cin >> s;
    bool used[26];
    memset(used, false, sizeof(used));
    for (char c : s) {
        used[c - 'a'] = true;
    }

    vector<char> v;
    for (int i = 0; i < 26; i++)
        if (!used[i])
            v.push_back(char(i + 'a'));

    memset(used, false, sizeof(used));
    int cnt = 0;
    for (char& c : s) {
        if (used[c - 'a']) {
            if (v.size() == 0) {
                cout << "-1" << endl;
                return 0;
            }
            c = v.back();
            v.pop_back();
            cnt++;
        }
        else {
            used[c - 'a'] = true;
        }
    }

    cout << cnt << endl;

    return 0;
}
