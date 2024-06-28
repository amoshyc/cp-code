#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int N, K;
string s;

int solve(char c) { // 回傳 max len of beautiful substring (all c)
    // two pointers
    // 當區間左端點固定時，右端點也是確定的。
    // 隨著左端點右移，右端點保證不會左移。
    // [st, ed)
    int st = 0, ed = 0;
    int cnt = 0, best = -1;
    for (;;) {
        while (ed < N && cnt + (s[ed] != c) <= K) {
            cnt += (s[ed] != c);
            ed++;
        }

        int len = ed - st;
        best = max(best, len);

        if (ed >= N)
            return best;

        cnt -= (s[st] != c);
        st++;
    }

    return -1;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> K >> s;
    cout << max(solve('a'), solve('b')) << endl;
    // solve('a');

    return 0;
}
