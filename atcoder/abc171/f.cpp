// K = int(input())
// S = input()
// M = int(10**9 + 7)

// fact = [1]
// for i in range(1, K + len(S) + 10):
//     fact.append(fact[-1] * i % M)

// finv = [pow(fact[-1], M - 2, M)]
// for i in range(K + len(S) + 9, 0, -1):
//     finv.append(finv[-1] * i % M)
// finv.reverse()

// def comb(a, b, m):
//     return fact[a] * finv[b] % m * finv[a - b] % m

// def hcomb(a, b, m):
//     return comb(a + b - 1, a - 1, m)

// ans = 0
// for l in range(0, K + 1):
//     ans += pow(26, l, M) * pow(25, K - l, M) % M * hcomb(len(S), K - l, M) % M
//     ans %= M
// print(ans)

#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

typedef long long ll;
const ll M = 1000000007;


struct CombTool {
    vector<ll> fact;
    vector<ll> finv;

    CombTool(ll max_value) {
        fact = vector<ll>();
        fact.push_back(1);
        for (int i = 1; i < max_value; i++) {
            fact.push_back(fact.back() * i % M);
        }

        finv = vector<ll>();
        finv.push_back(fast_pow(fact.back(), M - 2));
        for (int i = max_value - 1; i > 0; i--) {
            finv.push_back(finv.back() * i % M);
        }
        reverse(finv.begin(), finv.end());
    }

    ll comb(ll a, ll b) {
        return fact[a] * finv[b] % M * finv[a - b] % M;
    }

    ll hcomb(ll a, ll b) {
        return comb(a + b - 1, a - 1);
    }

    ll fast_pow(ll a, ll b) {
        ll res = 1;
        ll base = a;
        while (b) {
            if (b & 1) {
                res = res * base % M;
            }
            base = base * base % M;
            b = b / 2;
        }
        return res;
    }
};



int main() {
    ios::sync_with_stdio(false); 
    cin.tie(NULL);

    int K; string s;
    cin >> K >> s;

    auto tool = CombTool(K + int(s.size()) + 10);
    ll ans = 0;
    for (int l = 0; l <= K; l++) {
        ans += tool.fast_pow(26, l) * tool.fast_pow(25, K - l) % M * tool.hcomb(s.size(), K - l) % M;
        ans %= M;
    }
    cout << ans << endl;

    return 0;
}