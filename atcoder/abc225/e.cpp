#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

struct Slope {
    int p, q;

    bool operator<(const Slope &other) const {
        return 1ll * other.q * p < 1ll * q * other.p;
    }

    bool operator==(const Slope &other) const {
        if (1ll * other.q * p == 1ll * q * other.p)
            return true;
        return false;
    }
};

using Seg = pair<Slope, Slope>;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int N;
    cin >> N;

    vector<Seg> segs;
    for (int i = 0; i < N; i++) {
        int x, y;
        cin >> x >> y;
        segs.push_back(Seg{
            Slope{y, x - 1}, // end
            Slope{y - 1, x}, // start
        });
    }

    sort(segs.begin(), segs.end());

    int ans = 1;
    int j = 0;
    for (int i = 1; i < N; i++) {
        auto [fi, si] = segs[i];
        auto [fj, sj] = segs[j];
        if (fj < si || fj == si) {
            ans++;
            j = i;
        }
    }

    cout << ans << endl;

    return 0;
}