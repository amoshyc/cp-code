#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using ll = long long;
using mint = atcoder::modint1000000007;
using vec = vector<mint>;
using mat = vector<vec>;

mat matmul(mat A, mat B) {
    int n = A.size();
    int m = B[0].size();
    int l = B.size();
    mat C(n, vec(m, 0));
    for (int r = 0; r < n; r++) {
        for (int c = 0; c < m; c++) {
            for (int k = 0; k < l; k++) {
                C[r][c] += A[r][k] * B[k][c];
            }
        }
    }
    return C;
}

mat matpow(mat A, ll k) {
    int n = A.size();
    auto base = A;
    auto res = mat(n, vec(n, 0));
    for (int i = 0; i < n; i++) {
        res[i][i] = 1;
    }
    while (k) {
        if (k & 1) {
            res = matmul(res, base);
        }
        base = matmul(base, base);
        k >>= 1;
    }
    return res;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll N, K;
    cin >> N >> K;

    auto A = mat(N, vec(N, 0));
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            int e;
            cin >> e;
            A[r][c] = e;
        }
    }

    auto W = matpow(A, K);
    mint ans = 0;
    for (int r = 0; r < N; r++) {
        for (int c = 0; c < N; c++) {
            ans += W[r][c];
        }
    }

    cout << ans.val() << endl;

    return 0;
}