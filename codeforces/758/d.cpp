#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
#define sz(x) (int(x.size()))

ll ans = 1e18 + 1;
ll base;
string s;

inline ll to_ll(string a) {
    istringstream iss(a);
    ll res; iss >> res;
    return res;
}

void dfs(int idx, ll val) {
    // cout << ind << idx << ": " << val << endl;

    if (idx == sz(s)) {
        ans = min(ans, val);
        return;
    }

    if (val >= ll(1e18) / base || val >= ans) {
        return;
    }

    if (s[idx] == '0' && idx != sz(s) - 1) {
        dfs(idx + 1, val * base + s[idx] - '0');
        return;
    }

    for (int t = idx; t < sz(s); t++) {
        ll c = to_ll(s.substr(idx, t - idx + 1));
        if (c >= base) break;
        dfs(t + 1, val * base + c);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> base;
    cin >> s;

    dfs(0, 0);
    cout << ans << endl;

    return 0;
}
