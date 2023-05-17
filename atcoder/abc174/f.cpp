#include <iostream>
#include <vector>
#include <tuple>
#include <algorithm>
using namespace std;

template<class T>
struct BIT {
    int N;
    vector<T> data;
    BIT(int N) {
        data = vector<T>(N + 1);
        this->N = N;
    }

    void add_at(int idx, T val) {
        for (int i = idx + 1; i <= N; i += (i & (-i))) {
            data[i] += val;
        }
    }

    T sum_to(int idx) {
        T res = 0;
        for (int i = idx + 1; i > 0; i -= (i & (-i))) {
            res += data[i];
        }
        return res;
    }

    T sum_in(int l, int r) { // [l, r]
        return sum_to(r) - sum_to(l - 1);
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, Q; 
    cin >> N >> Q;

    vector<int> C(N, -1);
    for (int i = 0; i < N; i++) {
        cin >> C[i];
    }

    vector<vector<tuple<int, int, int>>> queries(N);
    for (int i = 0; i < Q; i++) {
        int l, r;
        cin >> l >> r;
        l--; r--;
        queries[r].push_back(tuple<int, int, int>(l, r, i));
    }

    auto bit = BIT<int>(N);
    auto V = *max_element(C.begin(), C.end());
    auto last = vector<int>(V + 1, -1);
    auto ans = vector<int>(Q, -1);

    for (int i = 0; i < N; i++) {
        if (last[C[i]] != -1) {
            bit.add_at(last[C[i]], -1);
        }

        bit.add_at(i, +1);
        last[C[i]] = i;

        for (auto& [l, r, idx] : queries[i]) {
            ans[idx] = bit.sum_in(l, r);
        }
    }

    for (auto x: ans) {
        cout << x << endl;
    }

    return 0;
}