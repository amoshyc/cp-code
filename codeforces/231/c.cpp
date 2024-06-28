#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const int MAX_N = 100000;

int N; ll K;
ll A[MAX_N];
ll S[MAX_N];
int ans_val = -1;

// [s, t)
ll sum(int s, int t) {
    return ((s == 0) ? S[t - 1] : (S[t - 1] - S[s - 1]));
}

bool C(ll x) {
    // A 已排序
    // 變成詢問 A 中存不存一個長度為 x 的連續區間，
    // 只需要最多 K 次操作即可變成相同值（區間最後一項）

    for (int s = 0; s + x - 1 < N; s++) {
        ll need = x * A[s + x - 1] - sum(s, s + x);
        if (need <= K) {
            ans_val = A[s + x - 1];
            return true;
        }
    }

    return false;
}

void solve() {
    sort(A, A + N);
    S[0] = A[0];
    for (int i = 1; i < N; i++)
        S[i] = S[i - 1] + A[i];

    // binary search
    // C(x) = 可否使用最多 K 次操作，產生 x 個相同值
    // 1 1 1 1 0 0 0
    ll lb = 1, ub = N;
    // if (!C(lb)) impossible;
    if (C(ub)) {
        printf("%d %lld\n", N, A[N - 1]);
        return;
    }

    while (ub - lb > 1) {
        ll mid = (lb + ub) / 2;
        if (C(mid)) lb = mid;
        else ub = mid;
    }

    C(lb);

    printf("%lld %d\n", lb, ans_val);
}

int main() {
    scanf("%d %lld", &N, &K);
    for (int i = 0; i < N; i++) {
        scanf("%lld", &A[i]);
    }

    solve();

    return 0;
}
