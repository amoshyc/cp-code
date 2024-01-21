#include <algorithm>
#include <atcoder/modint>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;

using mint = atcoder::modint998244353;

template <typename T> struct CombTool {
    vector<T> fact;
    vector<T> finv;

    CombTool(int V) {
        fact = vector<T>(V, 1);
        finv = vector<T>(V, 1);
        for (int i = 1; i < V; i++) {
            fact[i] = fact[i - 1] * i;
        }
        finv[V - 1] = fact.back().inv();
        for (int i = V - 2; i >= 0; i--) {
            finv[i] = finv[i + 1] * (i + 1);
        }
    }

    T comb(int a, int b) { return fact[a] * finv[b] * finv[a - b]; }
    T perm(int a, int b) { return fact[a] * finv[a - b]; }
    T hcomb(int a, int b) { return comb(a + b - 1, b); }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, K;
    cin >> N >> K;
    auto A = vector<int>(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    auto tool = CombTool<mint>(300 + 10);
    auto sum_of_pow = vector<mint>(K + 1, 0);
    sum_of_pow[0] = N;
    for (int i = 0; i < N; i++) {
        mint a = A[i];
        for (int c = 1; c <= K; c++) {
            sum_of_pow[c] += a;
            a = a * A[i];
        }
    }

    for (int x = 1; x <= K; x++) {
        mint ans = 0;
        for (int k = 0; k <= x; k++) {
            mint val = 0;
            val = val + sum_of_pow[x - k] * sum_of_pow[k] - sum_of_pow[x];
            val = val * tool.comb(x, k);
            ans += val;
        }
        ans = ans / 2;
        cout << ans.val() << endl;
    }

    return 0;
}