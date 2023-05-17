#include <atcoder/fenwicktree>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint1000000007;
using BIT = atcoder::fenwick_tree<mint>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, K;
    cin >> N >> K;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    auto dp = vector<BIT>(N, BIT(K + 1));
    for (int j = 0; j <= A[0]; j++) {
        dp[0].add(j, +1);
    }

    for (int i = 1; i < N; i++) {
        for (int j = 0; j <= K; j++) {
            dp[i].add(j, dp[i - 1].sum(j - min(A[i], j), j + 1));
        }
    }

    cout << dp[N - 1].sum(K, K + 1).val() << endl;

    return 0;
}