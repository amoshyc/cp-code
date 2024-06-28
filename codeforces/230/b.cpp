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

int main() {
    init_primes();

    int N;
    scanf("%d", &N);

    for (int i = 0; i < N; i++) {
        ll inp;
        scanf("%lld", &inp);
        ll sq = floor(sqrt(double(inp)));
        puts(((sq * sq == inp && is_prime[sq]) ? "YES" : "NO"));
    }

    return 0;
}
