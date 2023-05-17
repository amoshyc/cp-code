#include <iostream>
#include <set>
#include <vector>
using namespace std;

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << " ";
    }
    out << "]";
    return out;
}

using ll = long long;


int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int Q;
    cin >> Q;

    const ll N = (1 << 20);
    vector<ll> A(N, -1);
    set<ll> indices;
    for (int i = 0; i < N; i++) {
        indices.insert(i);
    }

    while (Q--) {
        ll t, x;
        cin >> t >> x;

        if (t == 1) {
            ll h = x % N;
            auto lb = indices.lower_bound(h);
            if (lb == indices.end()) {
                lb = indices.begin();
            }
            int idx = *lb;
            A[idx] = x;
            indices.erase(lb);
        } else {
            cout << A[x % N] << "\n";
        }
    }


    return 0;
}