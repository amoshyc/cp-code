#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, A;
    cin >> N >> A;

    vector<ll> x(N, 0);
    for (int i = 0; i < N; i++)
        cin >> x[i];

    if (N == 1) {
        cout << "0" << endl;
        return 0;
    }

    sort(x.begin(), x.end());

    ll ans1 = (x[N - 1] - x[1]) + min(abs(x[N - 1] - A), abs(x[1] - A));
    ll ans2 = (x[N - 2] - x[0]) + min(abs(x[N - 2] - A), abs(x[0] - A));
    cout << min(ans1, ans2) << endl;

    return 0;
}
