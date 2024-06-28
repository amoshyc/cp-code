#include <bits/stdc++.h>
using namespace std;

string s;

bool C(int k) {
    int N = s.size();
    
    vector<int> cnt(26, 0);
    vector<bool> isDom(26, false);

    for (int i = 0; i < k; i++) {
        cnt[s[i] - 'a']++;
        isDom[s[i] - 'a'] = true;
    }

    for (int i = 1; i + k - 1 < N; i++) {
        cnt[s[i - 1] - 'a']--;
        cnt[s[i + k - 1] - 'a']++;
        for (int c = 0; c < 26; c++) {
            if (cnt[c] == 0) {
                isDom[c] = false;
            }
        }
    }

    return find(isDom.begin(), isDom.end(), true) != isDom.end();
}

int solve() {
    // 0 0 0 1 1 1
    int lb = 1, ub = s.size();
    if (C(lb)) return lb;
    // if (!C(ub)) impossible
    while (ub - lb > 1) {
        int m = (lb + ub) / 2;
        if (C(m)) ub = m;
        else lb = m;
    }
    return ub;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cin >> s;
    cout << solve() << endl;
    return 0;
}