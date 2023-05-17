#include <algorithm>
#include <atcoder/lazysegtree>
#include <iostream>
#include <vector>
using namespace std;

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    for (auto x : v) {
        out << x << " ";
    }
    return out;
}

using ll = long long;
using S = ll;
using F = ll;
S op(S a, S b) { return a + b; }
S default_data() { return 0ll; }
S apply_lazy(F lazy, S data) { return data + lazy; }
F push_lazy(F root, F child) { return child + root; }
F default_lazy() { return 0ll; }
using SegTree = atcoder::lazy_segtree<S, op, default_data, F, apply_lazy,
                                      push_lazy, default_lazy>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll N, K;
    cin >> N >> K;

    auto seg = SegTree(2 * N + 10);
    for (ll m = 1; m <= N; m++) {
        ll lb = 1;
        ll ub = min(2 * N - m, N);
        // cout << m << ":" << lb << " " << ub << endl;
        seg.apply(m + lb, m + ub + 1, 1);
    }

    // for (int i = 2; i <= 2 * N; i++) {
    //     cout << i << ": " << seg.get(i) << endl;
    // }

    ll ans = 0;
    for (ll alpha = 2; alpha <= 2 * N; alpha++) {
        ll beta = alpha + K;
        if (2 <= beta && beta <= 2 * N) {
            ans += seg.get(alpha) * seg.get(beta);
        }
    }
    cout << ans << endl;

    return 0;
}