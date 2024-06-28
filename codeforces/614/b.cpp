#include <bits/stdc++.h>
using namespace std;

typedef pair<bool, int> pbi;

pbi is_beautiful(string s) {
    const int N = s.length();

    int pos = find(s.begin(), s.end(), '1') - s.begin();
    if (pos == N)
        return pbi(false, -1);

    for (int i = pos + 1; i < N; i++)
        if (s[i] != '0')
            return pbi(false, -1);

    return pbi(true, N - pos - 1);

}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; cin >> N;
    int cnt = 0;
    string s = "1";
    for (int i = 0; i < N; i++) {
        string inp; cin >> inp;

        if (inp == "0") {
            cout << "0\n";
            return 0;
        }

        // is beautiful
        pbi flag = is_beautiful(inp);
        // cout << inp << ": " << flag.first << " " << flag.second << endl;
        if (flag.first) // beautiful
            cnt += flag.second;
        else
            s = inp;
    }

    cout << s;
    for (int i = 0; i < cnt; i++)
        cout << "0";
    cout << "\n";

    return 0;
}
