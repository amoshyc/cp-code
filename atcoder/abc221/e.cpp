#include <algorithm>
#include <atcoder/fenwicktree>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;
using mint = atcoder::modint998244353;
using BIT = atcoder::fenwick_tree<mint>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    vector<int> xs(A);
    sort(xs.begin(), xs.end());
    xs.resize(unique(xs.begin(), xs.end()) - xs.begin());

    BIT bit(N);

    mint ans = 0;
    for (int i = 0; i < N; i++) {
        int lb = lower_bound(xs.begin(), xs.end(), A[i]) - xs.begin();
        mint cnt = mint(2).pow(i) * bit.sum(0, lb + 1);
        ans = ans + cnt;
        bit.add(lb, mint(2).pow(i + 1).inv());
    }

    cout << ans.val() << endl;

    return 0;
}