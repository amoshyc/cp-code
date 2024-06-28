#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

int N, K;
ll B[100000];
ll C[100000];
bool isC[100000];

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++)
        scanf("%lld", &B[i]);
    for (int i = 0; i < K; i++) {
        scanf("%lld", &C[i]);
        C[i]--;
        isC[C[i]] = true;
    }

    ll cap = 0;
    for (int i = 0; i < K; i++)
        cap += B[C[i]];

    ll total = accumulate(B, B + N, 0ll);
    ll ans = 0;
    for (int i = 0; i < N; i++) {
        ll ref_sum = 0;
        int pid = ((i - 1) + N) % N;
        int nid = (i + 1) % N;

        if (isC[i]) {
            ref_sum = total - B[i];
        }
        else {
            ref_sum = cap + B[pid] + B[nid];
            if (isC[pid])
                ref_sum -= B[pid];
            if (isC[nid])
                ref_sum -= B[nid];
        }

        // printf("%d: %lld\n", i, ref_sum);

        ans += ref_sum * B[i];
    }

    printf("%lld\n", ans / 2);

    return 0;
}