#include <algorithm>
#include <atcoder/segtree>
#include <iostream>
#include <vector>

using namespace std;

int op(int a, int b) { return max(a, b); }
int default_data() { return 0; }
using SegTree = atcoder::segtree<int, op, default_data>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, K;
    cin >> N >> K;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    // dp[i] = the max length of B where B[-1] = i
    const int M = 300000 + 10;
    auto dp = SegTree(M);
    dp.set(A[0], 1);
    for (int i = 1; i < N; i++) {
        auto lb = max(A[i] - K, 0);
        auto ub = min(A[i] + K + 1, M);
        dp.set(A[i], dp.prod(lb, ub) + 1);
    }
    cout << dp.prod(0, M) << endl;

    return 0;
}