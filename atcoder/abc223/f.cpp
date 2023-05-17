#include <algorithm>
#include <atcoder/lazysegtree>
#include <iostream>
#include <vector>
using namespace std;

using S = int;
using F = int;
S op(S a, S b) { return min(a, b); }
S default_val() { return 0x3f3f3f3f; }
S apply_lazy(F lazy, S val) { return lazy + val; }
F push_lazy(F parent, F child) { return parent + child; }
F default_lazy() { return 0; }
using SegTree = atcoder::lazy_segtree<S, op, default_val, F, apply_lazy,
                                      push_lazy, default_lazy>;

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    out << "[ ";
    for (auto x : v) {
        out << x << " ";
    }
    out << "]";
    return out;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N, Q;
    cin >> N >> Q;
    string S;
    cin >> S;

    vector<int> P(N, 0);
    P[0] = ((S[0] == '(') ? +1 : -1);
    for (int i = 1; i < N; i++) {
        P[i] = P[i - 1] + ((S[i] == '(') ? +1 : -1);
    }

    SegTree seg(P);

    while (Q--) {
        int cmd, l, r;
        cin >> cmd >> l >> r;
        l--;
        r--;
        if (cmd == 1) {
            if (S[l] == '(' and S[r] == ')') {
                seg.apply(l, r, -2);
                swap(S[l], S[r]);
            } else if (S[l] == ')' and S[r] == '(') {
                seg.apply(l, r, +2);
                swap(S[l], S[r]);
            }
        } else { // closed interval [l, r]
            // endpoint check
            bool check1 = S[l] == '(' and S[r] == ')';
            // bracket balance
            bool check2 = seg.get(r) - ((l > 0) ? seg.get(l - 1) : 0) == 0;
            // cannot drop under P[l] - 1
            bool check3 = seg.prod(l, r + 1) == seg.get(l) - 1;

            if (check1 and check2 and check3) {
                cout << "Yes" << endl;
            } else {
                cout << "No" << endl;
            }
        }
    }

    return 0;
}