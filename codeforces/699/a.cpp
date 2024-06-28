#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
typedef long long ll;
typedef pair<int, int> pii;
const ll INF = 1e18;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    string dir;
    cin >> dir;

    vector<ll> L;
    vector<ll> R;
    for (int i = 0; i < N; i++) {
        int d; cin >> d;
        if (dir[i] == 'L')
            L.push_back(d);
        else
            R.push_back(d);
    }

    ll ans = INF;
    for (ll d1 : R) {
        auto lb = lower_bound(L.begin(), L.end(), d1);
        if (lb == L.end()) continue;

        ll d2 = *lb;
        if (d2 < d1) continue;

        // cout << d1 << ", " << d2 << endl;

        ans = min(ans, (d2 - d1) / 2);
    }

    if (ans == INF) {
        cout << "-1\n";
    }
    else {
        cout << ans << "\n";
    }

    return 0;
}
