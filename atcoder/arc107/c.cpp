#include <algorithm>
#include <atcoder/dsu>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

template <class T> ostream &operator<<(ostream &out, const vector<T> v) {
    for (auto x : v) {
        out << x << " ";
    }
    return out;
}

using mint = atcoder::modint998244353;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, K;
    cin >> N >> K;

    auto A = vector<vector<int>>(N, vector<int>(N));
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            cin >> A[r][c];
        }
    }

    vector<mint> fact(N + 1);
    fact[0] = mint(1);
    for (int i = 1; i <= N; i++) {
        fact[i] = fact[i - 1] * i;
    }

    auto row_dsu = atcoder::dsu(N);
    for (int r1 = 0; r1 < N; r1++) {
        for (int r2 = r1 + 1; r2 < N; r2++) {
            bool all = true;
            for (int c = 0; c < N; c++) {
                if (A[r1][c] + A[r2][c] > K) {
                    all = false;
                    break;
                }
            }
            if (all) {
                row_dsu.merge(r1, r2);
            }
        }
    }

    auto col_dsu = atcoder::dsu(N);
    for (int c1 = 0; c1 < N; c1++) {
        for (int c2 = c1 + 1; c2 < N; c2++) {
            bool all = true;
            for (int r = 0; r < N; r++) {
                if (A[r][c1] + A[r][c2] > K) {
                    all = false;
                    break;
                }
            }
            if (all) {
                col_dsu.merge(c1, c2);
            }
        }
    }

    mint cntR = 1;
    for (const auto &group : row_dsu.groups()) {
        cntR = cntR * fact[group.size()];
    }
    mint cntC = 1;
    for (const auto &group : col_dsu.groups()) {
        cntC = cntC * fact[group.size()];
    }
    cout << (cntR * cntC).val() << endl;

    return 0;
}