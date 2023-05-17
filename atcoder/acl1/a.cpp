#include <algorithm>
#include <atcoder/dsu>
#include <iostream>
#include <map>
#include <tuple>
#include <vector>
using namespace std;


template <class K, class V>
ostream &operator<<(ostream &out, const map<K, V> m) {
    out << "Dict(size=" << m.size() << ", ";
    out << "{ ";
    for (auto &[k, v] : m) {
        out << k << ": " << v << ", ";
    }
    out << "})";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    vector<tuple<int, int, int>> towns(N);
    for (int i = 0; i < N; i++) {
        int r, c;
        cin >> r >> c;
        --r;
        --c;
        towns[i] = {i, r, c};
    }

    sort(towns.begin(), towns.end(), [](auto town_a, auto town_b) {
        auto [id1, r1, c1] = town_a;
        auto [id2, r2, c2] = town_b;
        return r1 < r2;
    });

    auto components = atcoder::dsu(N);
    auto pivot_cols = map<int, int>(); // col -> id

    for (int i = 0; i < N; i++) {
        auto [cur_id, cur_r, cur_c] = towns[i];
        auto lb = pivot_cols.lower_bound(cur_c);
        if (lb == pivot_cols.begin()) {
            pivot_cols[cur_c] = cur_id;
        } else {
            int min_col = cur_c;
            for (auto it = pivot_cols.begin(); it != lb; ++it) {
                auto [c, id] = *it;
                min_col = min(min_col, c);
                components.merge(cur_id, id);
            }
            pivot_cols.erase(pivot_cols.begin(), lb);
            pivot_cols[min_col] = cur_id;
        }
        // cout << pivot_cols << endl;
    }

    for (int i = 0; i < N; i++) {
        cout << components.size(i) << endl;
    }

    return 0;
}