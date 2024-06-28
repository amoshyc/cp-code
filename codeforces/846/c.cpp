#include <bits/stdc++.h>
using namespace std;

#define sz(x) (int(x.size()))
typedef long long ll;

const int MAX_N = 5000;
const ll INF = 1e16;
int N;
ll A[MAX_N];
ll S[MAX_N];

ll sum(int i, int j) { // [i, j)
    if (i == j) {
        return 0;
    }
    if (i == 0) {
        return S[j - 1];
    }
    return S[j - 1] - S[i - 1];
}

void solve() {
    // build prefix sum
    S[0] = A[0];
    for (int i = 1; i < N; i++) {
        S[i] = S[i - 1] + A[i];
    }

    // solve
    ll ans = -INF;
    int ans_i, ans_j, ans_k;

    for (int pivot_j = 0; pivot_j <= N; pivot_j++) {
        ll mx_left = -INF;
        ll mx_right = -INF;
        int pivot_i = -1, pivot_k = -1;

        for (int i = 0; i <= pivot_j; i++) {
            ll left = sum(0, i) - sum(i, pivot_j);
            // printf("%d, %d\t%lld, %lld\n", i, pivot_j, left, mx_left);
            
            if (left > mx_left) {
                mx_left = left;
                pivot_i = i;
            }
        }
        for (int k = pivot_j; k <= N; k++) {
            ll right = sum(pivot_j, k) - sum(k, N);
            if (right > mx_right) {
                mx_right = right;
                pivot_k = k;
            }
        }

        ll value = mx_left + mx_right;
        if (value > ans) {
            ans = value;
            ans_i = pivot_i;
            ans_j = pivot_j;
            ans_k = pivot_k;
        }
    }

    // output
    printf("%d %d %d\n", ans_i, ans_j, ans_k);
}

int main() {
    while (scanf("%d", &N) != EOF) {
        for (int i = 0; i < N; i++) {
            scanf("%lld", &A[i]);
        }
        solve();
    }
    return 0;
}
