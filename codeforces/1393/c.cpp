#include <vector>
#include <iostream>
#include <algorithm>
#include <map>

using namespace std;
typedef long long ll;

ll solve() {
    ll N;
    cin >> N;

    map<ll, ll> cnt;
    for (ll i = 0; i < N; i++) {
        ll x; cin >> x;
        cnt[x] += 1;
    }

    vector<ll> freq;
    for (auto p: cnt) {
        freq.push_back(p.second);
    }
    sort(freq.begin(), freq.end());
    reverse(freq.begin(), freq.end());

    auto check = [&](ll m) -> bool {
        ll idx = 0;
        for (auto num: freq) {
            ll last = (num - 1) * (m + 1) + idx;
            if (last >= N) {
                return false;
            }
            idx += 1;
        }
        return true;
    };

    ll lb = - 1;
    ll ub = N;

    // 1 1 1 0 0 0 
    while (ub - lb > 1) {
        ll m = (lb + ub) / 2;
        if (check(m)) {
            lb = m;
        }
        else {
            ub = m;
        }
    }

    return lb;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll TC;
    cin >> TC;

    while (TC--) {
        cout << solve() << endl;
    }

    return 0;
}