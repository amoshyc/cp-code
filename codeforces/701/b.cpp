#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int MAX_N = 1e5;
bool usedR[MAX_N];
bool usedC[MAX_N];

int main() {
    ll N, M;
    scanf("%lld %lld", &N, &M);

    ll rest = N * N;
    ll usedRcnt = 0, usedCcnt = 0;
    for (int i = 0; i < M; i++) {
        int r, c; scanf("%d %d", &r, &c);
        r--; c--;

        if (!usedR[r]) {
            rest -= (N - usedCcnt);
            usedRcnt++;
            usedR[r] = true;
        }
        if (!usedC[c]) {
            rest -= (N - usedRcnt);
            usedCcnt++;
            usedC[c] = true;
        }

        printf("%lld ", rest);
    }
    puts("");

    return 0;
}
