#include <atcoder/segtree>
#include <iostream>
#include <vector>
using namespace std;

using S = int;
S default_data() { return 0; }
S aggregate(S a, S b) { return a ^ b; }
using SegTree = atcoder::segtree<S, aggregate, default_data>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, Q;
    cin >> N >> Q;
    vector<int> A(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }

    auto seg = SegTree(A);
    while (Q--) {
        int t, x, y;
        cin >> t >> x >> y;
        if (t == 1) {
            seg.set(x - 1, seg.get(x - 1) ^ y);
        } else {
            cout << seg.prod(x - 1, y) << "\n";
        }
    }

    return 0;
}