#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll INF = 1e18;

int N, M;
vector<ll> A;
vector<ll> B;

int main() {
    scanf("%d %d", &N, &M);

    A = vector<ll>(N, 0);
    B = vector<ll>(M, 0);

    for (int i = 0; i < N; i++)
        scanf("%lld", &A[i]);
    for (int i = 0; i < M; i++)
        scanf("%lld", &B[i]);

    sort(A.begin(), A.end());
    sort(B.begin(), B.end());

    ll all_need = -1;
    for (ll x : A) {
        auto lb = lower_bound(B.begin(), B.end(), x);

        ll need = INF;
        if (lb != B.end()) {
            need = min(need, abs(*lb - x));
        }

        if (lb != B.begin()) {
            lb--;
            need = min(need, abs(x - *lb));
        }

        all_need = max(all_need, need);
    }

    printf("%lld\n", all_need);

    return 0;
}
