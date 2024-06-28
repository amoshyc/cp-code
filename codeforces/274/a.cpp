#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 100000;
int N, K;
ll A[MAX_N];

int main() {
    scanf("%d %d", &N, &K);
    for (int i = 0; i < N; i++)
        scanf("%lld", &A[i]);

    if (K == 1) {
        printf("%d\n", N);
        return 0;
    }

    set<ll> S;
    sort(A, A + N);
    for (int i = 0; i < N; i++) {
        if (A[i] % K != 0) {
            S.insert(A[i]);
            continue;
        }

        if (S.find(A[i] / K) == S.end()) {
            S.insert(A[i]);
            continue;
        }
    }

    printf("%lu\n", S.size());

    return 0;
}
