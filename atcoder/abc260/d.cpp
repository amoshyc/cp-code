#include <algorithm>
#include <iostream>
#include <map>
#include <vector>
using namespace std;

template <typename K, typename V>
ostream &operator<<(ostream &out, const map<K, V> m) {
    out << "{ ";
    for (auto &[k, v] : m) {
        out << k << ": " << v << ", ";
    }
    out << "}";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, K;
    cin >> N >> K;
    vector<int> P(N);
    for (int i = 0; i < N; i++) {
        cin >> P[i];
        P[i]--;
    }

    vector<int> ans(N, -1);
    vector<int> prev(N, -1);
    if (K == 1) {
        for (int i = 0; i < N; i++) {
            ans[P[i]] = i;
        }

        for (int x : ans) {
            if (x != -1)
                cout << x + 1 << endl;
            else
                cout << -1 << endl;
        }
        return 0;
    }

    map<int, int> stacks;
    for (int i = 0; i < N; i++) {
        auto lb = stacks.lower_bound(P[i]);

        if (lb == stacks.end()) {
            stacks[P[i]] = K - 1;
        } else {
            int key = lb->first;
            int cnt = lb->second;
            stacks.erase(lb);
            if (cnt == 1) {
                ans[P[i]] = i;
                ans[key] = i;
                while (prev[key] != -1) {
                    key = prev[key];
                    ans[key] = i;
                }
            } else {
                prev[P[i]] = key;
                stacks[P[i]] = cnt - 1;
            }
        }
    }

    for (int x : ans) {
        if (x != -1)
            cout << x + 1 << endl;
        else
            cout << -1 << endl;
    }

    return 0;
}