#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 300000;

int N, M;
int A[MAX_N];
int pos[MAX_N + 1];
int dp[MAX_N];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> M;
    for (int i = 0; i < N; i++) {
        cin >> A[i];
        pos[A[i]] = i;
    }

    memset(dp, -1, sizeof(dp));
    while (M--) {
        int a, b; cin >> a >> b;
        if (pos[a] > pos[b])
            swap(a, b);

        dp[pos[b]] = max(dp[pos[b]], pos[a]);
    }

    ll ans = 0;
    int pre = -1;
    for (int i = 0; i < N; i++) {
        ans += i - max(pre, dp[i]);
        pre = max(pre, dp[i]);
    }

    cout << ans << "\n";

    return 0; 
}
