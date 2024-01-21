#include <algorithm>
#include <iostream>
#include <set>
#include <utility>
#include <vector>
using namespace std;

using ll = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    ll N, T;
    cin >> N >> T;

    vector<ll> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    int N1 = N / 2;
    int N2 = N - N1;

    vector<ll> s1;
    for (int mask = 0; mask < (1 << N1); mask++) {
        ll sum = 0;
        for (int i = 0; i < N1; i++) {
            if (mask & (1 << i)) {
                sum += A[i];
            }
        }
        s1.push_back(sum);
    }

    vector<ll> s2;
    for (int mask = 0; mask < (1 << N2); mask++) {
        ll sum = 0;
        for (int i = 0; i < N2; i++) {
            if (mask & (1 << i)) {
                sum += A[N1 + i];
            }
        }
        s2.push_back(sum);
    }

    sort(s2.begin(), s2.end());

    ll ans = -1;
    for (auto sum1 : s1) {
        auto ub = upper_bound(s2.begin(), s2.end(), T - sum1);
        if (ub != s2.begin()) {
            ll sum2 = *(--ub);
            ans = max(ans, sum1 + sum2);
        }
    }

    cout << ans << endl;

    return 0;
}