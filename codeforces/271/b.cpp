#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const ll MAX_NUM = 1e6;
bool is_prime[MAX_NUM];
vector<ll> primes;

void init_primes() {
    fill(is_prime, is_prime + MAX_NUM, true);
    is_prime[0] = is_prime[1] = false;
    for (ll i = 2; i < MAX_NUM; i++) {
        if (is_prime[i]) {
            primes.push_back(i);
            for (ll j = i * i; j < MAX_NUM; j += i)
                is_prime[j] = false;
        }
    }
}

const int MAX_N = 500;
const int MAX_M = 500;
int N, M;
ll A[MAX_N][MAX_M];
ll B[MAX_N][MAX_M];

int main() {
    init_primes();

    scanf("%d %d", &N, &M);
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            scanf("%lld", &A[r][c]);
        }
    }

    for (int r = 0; r < N; r++) {
        for (int c = 0; c < M; c++) {
            ll p = *lower_bound(primes.begin(), primes.end(), A[r][c]);
            B[r][c] = p - A[r][c];
        }
    }

    ll ans = 1e18;
    // row
    for (int r = 0; r < N; r++) {
        ans = min(ans, accumulate(B[r], B[r] + M, 0ll));
    }
    // col
    for (int c = 0; c < M; c++) {
        ll sum = 0;
        for (int r = 0; r < N; r++) {
            sum += B[r][c];
        }
        ans = min(ans, sum);
    }

    printf("%lld\n", ans);

    return 0;
}
