#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int M = 1e9 + 7;

ll fast_pow(ll a, ll b) {
    ll ans = 1;
    ll base = a % M;
    while (b) {
        if (b & 1) ans = (ans * base) % M;
        base = (base * base) % M;
        b >>= 1;
    }
    return ans;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string s; cin >> s;
    int cnt = 0;
    for (char c : s) {
        ll digit = 0;
        if (isdigit(c)) digit = (c - '0');
        if (isupper(c)) digit = (c - 'A') + 10;
        if (islower(c)) digit = (c - 'a') + 36;
        if (c == '-') digit = 62;
        if (c == '_') digit = 63;

        for (int i = 0; i < 6; i++) {
            if ((digit & (1ll << i)) == 0) {
                cnt++;
            }
        }
    }

    cout << fast_pow(3, cnt) << "\n";

    return 0;
}
