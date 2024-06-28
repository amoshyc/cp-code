#include <bits/stdc++.h>
using namespace std;

// Generalized Triangle Inequality:
// The length of any polygon side is always less than
// the sum of the other polygon side lengths.
// i.e.
// The maximum length < sum of other lengths

// Now inequality is not holds, we need to add a length to other lengths
// Thus,
// ans = maximum length - sum of other lengths + 1

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    vector<ll> A(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    ll MM = *max_element(A.begin(), A.end());
    ll rest = accumulate(A.begin(), A.end(), 0ll) - MM;
    cout << (MM - rest + 1) << endl;

    return 0;
}
