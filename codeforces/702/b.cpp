#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
// const int INF = 0x3f3f3f3f;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    vector<ll> A(N, 0);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    sort(A.begin(), A.end());

    ll cnt = 0;
    for (ll val = 1; val < ll(1e18); val <<= 1) {
        for (int i = 0; i < N; i++) {
            auto lb = lower_bound(A.begin(), A.end(), val - A[i]);
            auto ub = upper_bound(A.begin(), A.end(), val - A[i]);

            if (lb == A.end())
                continue;

            if (val - A[i] == A[i]) {
                cnt += (ub - lb - 1);
            }
            else {
                cnt += (ub - lb);
            }
        }
    }

    cout << (cnt / 2) << endl;

    return 0;
}
