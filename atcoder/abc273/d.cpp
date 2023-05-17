#include <algorithm>
#include <iostream>
#include <set>
#include <unordered_map>
#include <vector>
using namespace std;
using ll = long long;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    ll H, W, r, c;
    cin >> H >> W >> r >> c;
    r--;
    c--;

    auto pivotsH = unordered_map<ll, set<ll>>();
    auto pivotsV = unordered_map<ll, set<ll>>();

    int N;
    cin >> N;
    for (int i = 0; i < N; i++) {
        int a, b;
        cin >> a >> b;
        a--;
        b--;
        pivotsH[a].insert(b);
        pivotsV[b].insert(a);
    }

    int Q;
    cin >> Q;
    while (Q--) {
        string d;
        ll l;
        cin >> d >> l;

        if (d == "U") {
            ll prev_r = -1;
            if (pivotsV.find(c) != pivotsV.end()) {
                auto it = pivotsV[c].lower_bound(r);
                if (it != pivotsV[c].begin()) {
                    prev_r = *(--it);
                }
            }
            r = max(r - l, prev_r + 1);
        }
        if (d == "D") {
            ll next_r = H;
            if (pivotsV.find(c) != pivotsV.end()) {
                auto it = pivotsV[c].upper_bound(r);
                if (it != pivotsV[c].end()) {
                    next_r = *it;
                }
            }
            r = min(r + l, next_r - 1);
        }
        if (d == "L") {
            ll prev_c = -1;
            if (pivotsH.find(r) != pivotsH.end()) {
                auto it = pivotsH[r].lower_bound(c);
                if (it != pivotsH[r].begin()) {
                    prev_c = *(--it);
                }
            }
            c = max(c - l, prev_c + 1);
        }
        if (d == "R") {
            ll next_c = W;
            if (pivotsH.find(r) != pivotsH.end()) {
                auto it = pivotsH[r].upper_bound(c);
                if (it != pivotsH[r].end()) {
                    next_c = *it;
                }
            }
            c = min(c + l, next_c - 1);
        }

        cout << r + 1 << " " << c + 1 << "\n";
    }

    return 0;
}