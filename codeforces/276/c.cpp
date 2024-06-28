#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 200000;

int N, Q;
int A[MAX_N];
int F[MAX_N + 1];
ll S[MAX_N];

int main() {
    scanf("%d %d", &N, &Q);

    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
    }

    for (int i = 0; i < Q; i++) {
        int l, r; scanf("%d %d", &l, &r); l--; r--;
        F[l]++;
        F[r + 1]--;
    }

    S[0] = F[0];
    for (int i = 1; i < N; i++) {
        S[i] = S[i - 1] + F[i];
    }

    sort(A, A + N);
    sort(S, S + N);

    ll ans = 0;
    for (int i = 0; i < N; i++) {
        ans += A[i] * S[i];
    }

    printf("%lld\n", ans);

    return 0;
}
