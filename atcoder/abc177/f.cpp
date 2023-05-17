#include <algorithm>
#include <iostream>
#include <map>
#include <set>
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

template <class T> ostream &operator<<(ostream &out, const multiset<T> s) {
    out << "Set(size=" << s.size() << ", ";
    out << "{ ";
    for (auto it = s.begin(); it != s.end(); ++it) {
        out << *it << " ";
    }
    out << "})";
    return out;
}

void solve() {
    int H, W;
    cin >> H >> W;

    multiset<int> steps;
    map<int, int> end2start;
    for (int c = 0; c < W; c++) {
        end2start[c] = c;
        steps.insert(0);
    }

    for (int r = 1; r <= H; r++) {
        int a, b;
        cin >> a >> b;
        a--;
        b--;

        // Erase pairs (e, s) that a <= e <= b + 1
        auto lb = end2start.lower_bound(a);
        auto ub = end2start.upper_bound(b + 1);
        auto max_start = -1;
        for (auto it = lb; it != ub; ++it) {
            int end = it->first;
            int start = it->second;
            steps.erase(steps.find(end - start));
            max_start = max(max_start, start);
        }
        end2start.erase(lb, ub);

        // Update pair (b + 1, s)
        if (b != W - 1 and max_start != -1) {
            end2start[b + 1] = max_start;
            steps.insert(b + 1 - end2start[b + 1]);
        }

        if (steps.size() > 0) {
            cout << *steps.begin() + r << endl;
        } else {
            cout << "-1" << endl;
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    solve();
    return 0;
}