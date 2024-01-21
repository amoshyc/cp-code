#include <iostream>
#include <vector>
#include <tuple>
#include <algorithm>
using namespace std;

typedef tuple<int, int, int> t3i;

template <class T>
struct BIT {
    int NN;
    vector<T> data;

    BIT(int N, T default_value) {
        NN = N;
        data = vector<T>(NN + 1, default_value);
    }

    void add_at(int idx, T val) {
        for (int i = idx + 1; i <= NN; i += (i & (-i))) {
            data[i] += val;
        }
    }

    T sum_to(int idx) { // [0, idx]
        T res = 0;
        for (int i = idx + 1; i >= 1; i -= (i & (-i))) {
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
    cin.tie(nullptr);

    int N, Q;
    cin >> N >> Q;
    
    auto C = vector<int>(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> C[i];
    }

    auto queries = vector<vector<t3i>>(N, vector<t3i>());
    for (int i = 0; i < Q; i++) {
        int l, r; 
        cin >> l >> r;
        l--; r--;
        queries[r].push_back({i, l, r});
    }

    auto ans = vector<int>(Q, -1);
    auto V = *max_element(C.begin(), C.end());
    auto last = vector<int>(V + 1, -1);
    
    auto bit = BIT<int>(N, 0);
    for (int i = 0; i < N; i++) {
        if (last[C[i]] != -1) {
            bit.add_at(last[C[i]], -1);
        }
        bit.add_at(i, +1);
        last[C[i]] = i;
        for (auto&& [id, l, r]: queries[i]) {
            ans[id] = bit.sum_in(l, r);
        }
    }

    for (auto x : ans) {
        cout << x << endl;
    }

    return 0;
}