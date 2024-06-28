#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))
typedef long long ll;

const ll MAX = ll(1e18);
int N;
vector<ll> bs;
vector<ll> as;

void solve() {
    as[0] = 0;
    as[N - 1] = bs[0];

    ll prev_a = 0;
    ll prev_b = bs[0];
    for (int i = 1; i < N / 2; i++) {
        ll a = prev_a;
        ll b = prev_b;
        ll sum = a + b;
        if (sum == bs[i]) {
            as[i] = a;
            as[N - i - 1] = b;
            continue;
        }
        if (sum < bs[i]) {
            ll need = bs[i] - sum;
            a += need;
        }
        else {
            ll over = sum - bs[i];
            b -= over;
        }
        as[i] = a;
        as[N - i - 1] = b;
        prev_a = a;
        prev_b = b;
    }

    for (int i = 1; i < N; i++) {
        assert (as[i] >= 0);
        assert (as[i - 1] <= as[i]);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    cin >> N;
    bs = vector<ll>(N, -1);
    as = vector<ll>(N, -1);
    for (int i = 0; i < N / 2; i++) {
        cin >> bs[i];
    }

    solve();
    for (ll x: as) {
        cout << x << " ";
    }
    cout << endl;

    return 0;
}
