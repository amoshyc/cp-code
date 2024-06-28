#include <iostream>
#include <set>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    set<int> xs;
    multiset<int> gaps;

    auto add_pile = [&](int x) {
        auto it_r = xs.lower_bound(x);
        if (it_r != xs.end() && it_r != xs.begin()) {
            gaps.erase(gaps.find(*it_r - *prev(it_r)));
        }
        if (it_r != xs.end()) {
            gaps.insert(*it_r - x);
        }
        if (it_r != xs.begin()) {
            gaps.insert(x - *prev(it_r));
        }
        xs.insert(x);
    };

    auto del_pile = [&](int x) {
        xs.erase(x);
        auto it_r = xs.lower_bound(x);
        if (it_r != xs.end()) {
            gaps.erase(gaps.find(*it_r - x));
        }
        if (it_r != xs.begin()) {
            gaps.erase(gaps.find(x - *prev(it_r)));
        }
        if (it_r != xs.end() and it_r != xs.begin()) {
            gaps.insert(*it_r - *prev(it_r));
        }
    };

    auto query = [&]() -> int {
        if (xs.size() <= 1) {
            return 0;
        }
        return *xs.rbegin() - *xs.begin() - *gaps.rbegin();
    };

    int N, Q;
    cin >> N >> Q;
    for (int i = 0; i < N; i++) {
        int x;
        cin >> x;
        add_pile(x);
    }
    cout << query() << endl;

    while (Q--) {
        int cmd, x;
        cin >> cmd >> x;
        if (cmd == 0) {
            del_pile(x);
        } else {
            add_pile(x);
        }
        cout << query() << endl;
    }

    return 0;
}