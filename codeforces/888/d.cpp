#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const ll D[] = {1, 0, 1, 2, 9};

ll C(ll n, ll m) {
    m = min(m, n - m);
    ll ans = 1;
    for (int i = 1; i <= m; i++) {
        ans *= (n - i + 1);
        ans /= i;
    }
    return ans;
}

int main() {
    int N, K;
    cin >> N >> K;
    ll ans = 0;
    for (int i = N - K; i <= N; i++) {
        ans += C(N, i) * D[N - i];
    }
    cout << ans << endl;

    return 0;
}