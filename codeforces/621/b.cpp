#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int max_n = 200000;
int n;
ll cnt1[2 * max_n - 1];
ll cnt2[2 * max_n - 1];

int main() {
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        int r, c; scanf("%d %d", &r, &c);
        r--; c--;

        cnt1[r - c + (n - 1)]++;
        cnt2[r + c]++;
    }

    ll ans = 0;
    for (int i = 0; i < 2 * max_n - 1; i++)
        if (cnt1[i] > 0)
            ans += cnt1[i] * (cnt1[i] - 1) / 2;
    for (int i = 0; i < 2 * max_n - 1; i++)
        if (cnt2[i] > 0)
            ans += cnt2[i] * (cnt2[i] - 1) / 2;

    printf("%lld\n", ans);

    return 0;
}
