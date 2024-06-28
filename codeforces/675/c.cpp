#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll N; cin >> N;
    vector<int> data(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> data[i];
    }

    map<ll, int> cnt;
    ll sum = 0;
    ll ans = N - 1;
    for (int v : data) {
        sum += v;
        cnt[sum]++;
        ans = min(ans, N - cnt[sum]);
    }

    cout << ans << "\n";

    return 0;
}
