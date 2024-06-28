#include <bits/stdc++.h>
using namespace std;
// 
// bool is_palindrome(string s) {
//     int len = s.length();
//     for (int i = 0; i < len / 2; i++)
//         if (s[i] != s[len - i - 1])
//             return false;
//     return true;
// }

int main() {
    ios::sync_with_stdio(false);

    string inp; cin >> inp;

    // if (is_palindrome(inp)) {
    //     cout << inp << endl;
    //     return 0;
    // }

    int cnt[26]; memset(cnt, 0, sizeof(cnt));
    for (char c : inp)
        cnt[c - 'a']++;

    for (int i = 25; i >= 0; i--) {
        if (cnt[i] & 1) {
            for (int j = 0; j < i; j++) {
                if (cnt[j] & 1) {
                    cnt[j]++;
                    cnt[i]--;
                    break;
                }
            }
        }
    }

    string ans = "";
    for (int i = 0; i < 26; i++) {
        if (cnt[i] & 1) {
            ans += (i + 'a');
            cnt[i]--;
            break;
        }
    }
    for (int i = 25; i >= 0; i--) {
        if (cnt[i] != 0 && (cnt[i] & 1) == 0) {
            string s(cnt[i] / 2, char(i + 'a'));
            ans = s + ans + s;
        }
    }
    cout << ans << endl;

    return 0;
}
