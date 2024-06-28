#include <bits/stdc++.h>
using namespace std;

int N;
int cnt[10];

void output() {
    string res = "";
    for (int i = 9; i >= 1; i--) {
        res += string(cnt[i], char('0' + i));
    }

    if (res == "") res = "0";
    else {
        res += string(cnt[0], '0');
    }

    cout << res << "\n";
}

void solve() {
    if (cnt[0] == 0) {
        cout << "-1\n";
        return;
    }

    int sum = 0;
    for (int i = 1; i < 10; i++) {
        sum += cnt[i] * i;
    }

    if (sum % 3 == 1) {
        // remove 1 number which mod 3 is 1
        for (int i = 1; i < 10; i += 3) {
            if (cnt[i] > 0) {
                cnt[i]--;
                output();
                return;
            }
        }

        // remove 2 number which mod 3 is 2
        int n = 2;
        for (int i = 2; i < 10; i += 3) {
            if (n > 0 && cnt[i] > 0) {
                int val = min(cnt[i], n);
                cnt[i] -= val;
                n -= val;
            }
        }

        if (n > 0)
            cout << "-1\n";
        else
            output();
        return;
    }
    if (sum % 3 == 2) {
        // remove 1 number which mod 3 is 2
        for (int i = 2; i < 10; i += 3) {
            if (cnt[i] > 0) {
                cnt[i]--;
                output();
                return;
            }
        }

        // remove 2 number which mod 3 is 1
        int n = 2;
        for (int i = 1; i < 10; i += 3) {
            if (n > 0 && cnt[i] > 0) {
                int val = min(cnt[i], n);
                cnt[i] -= val;
                n -= val;
            }
        }

        if (n > 0)
            cout << "-1\n";
        else
            output();
        return;
    }

    output();
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N;
    for (int i = 0; i < N; i++) {
        int inp; cin >> inp;
        cnt[inp]++;
    }

    solve();

    return 0;
}
