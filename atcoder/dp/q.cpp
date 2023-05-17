#include <atcoder/segtree>
#include <iostream>
#include <vector>
using namespace std;

using ll = long long;
const ll INF = 1e18;
ll op(ll a, ll b) { return max(a, b); }
ll e() { return -INF; }
using SegTree = atcoder::segtree<ll, op, e>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<ll> h(N);
    vector<ll> a(N);
    for (int i = 0; i < N; i++) {
        cin >> h[i];
    }
    for (int i = 0; i < N; i++) {
        cin >> a[i];
    }

    vector<ll> dp(N);
    auto seg = SegTree(N + 10);

    dp[0] = a[0];
    seg.set(h[0], dp[0]);
    for (int i = 1; i < N; i++) {
        ll prev = seg.prod(0, h[i]);
        dp[i] = max(a[i], prev + a[i]);

        if (dp[i] > seg.get(h[i])) {
            seg.set(h[i], dp[i]);
        }
    }

    ll ans = *max_element(dp.begin(), dp.end());
    cout << ans << endl;

    return 0;
}