#include <algorithm>
#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

typedef long long ll;
typedef pair<ll, ll> pll;

struct SieveOfEratosthenes {
    vector<bool> is_prime;
    vector<ll> primes;

    SieveOfEratosthenes(ll V) {
        is_prime = vector<bool>(V + 1, true);
        primes = vector<ll>();
        for (ll i = 2; i <= V; i ++) {
            if (is_prime[i]) {
                primes.push_back(i);
                for (ll j = i * i; j <= V; j += i) {
                    is_prime[j] = false;
                }
            }
        }
    }

    vector<pll> factorize(ll x) {
        if (x <= 1) {
            throw invalid_argument("x shoule be > 1");
        }
        auto result = vector<pll>();
        for (auto p : primes) {
            ll exp = 0;
            while (x % p == 0) {
                exp++;
                x = x / p;
            }
            if (exp > 0) {
                result.push_back(pll(p, exp));
            }
            if (p * p > x) {
                break;
            }
        }
        if (x > 1) {
            result.push_back(pll(x, 1));
        }
        return result;
    }
};

ll gcd(ll a, ll b) {
    // gcd(x, 0) = x
    // gcd(a, b) = gcd(b, r)
    while (b != 0) {
        ll r = a % b;
        a = b;
        b = r;
    }
    return a;
}

string solve() {
    int N;
    cin >> N;
    vector<ll> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    if (all_of(A.begin(), A.end(), [&](const ll x) { return x == 1; })) {
        return "pairwise coprime";
    }


    ll gcd_all = A[0];
    for (int i = 1; i < N; i++) {
        gcd_all = gcd(gcd_all, A[i]);
    }

    if (gcd_all != 1) {
        return "not coprime";
    }
    
    auto sieve = SieveOfEratosthenes(*max_element(A.begin(), A.end()));
    auto used_primes = unordered_set<ll>();

    for (auto x : A) {
        if (x == 1) {
            continue;
        }
        auto factors = sieve.factorize(x);
        for (auto [p, e] : factors) {
            if (used_primes.find(p) != used_primes.end()) {
                return "setwise coprime";
            }
            used_primes.insert(p);
        }
    }

    return "pairwise coprime";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    cout << solve() << endl;

    return 0;
}