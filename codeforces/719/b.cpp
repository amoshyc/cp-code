#include <bits/stdc++.h>
using namespace std;

int N;
string s;

int solve(char even, char odd) {
    int cnt0 = 0, cnt1 = 0;
    for (int i = 0; i < N; i++) {
        if (i % 2 == 0 && s[i] != even)
            cnt0++;
        if (i % 2 == 1 && s[i] != odd)
            cnt1++;
    }

    // cout << cnt0 << " " << cnt1 << endl;

    return max(cnt0, cnt1);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> s;
    cout << min(solve('r', 'b'), solve('b', 'r')) << endl;

    return 0;
}
