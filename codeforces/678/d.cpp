#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef vector<ll> vec;
typedef vector<vec> mat;

ll M = 1e9 + 7;

mat mul(const mat& a, const mat& b) {
    mat res(a.size(), vec(b[0].size(), 0));
    for (int r = 0; r < a.size(); r++)
        for (int c = 0; c < b[0].size(); c++)
            for (int k = 0; k < b.size(); k++)
                res[r][c] = (res[r][c] + a[r][k] * b[k][c] % M) % M;

    return res;
}

mat pow(const mat& a, ll b) {
    mat ans(a.size(), vec(a.size(), 0));
    for (int i = 0; i < a.size(); i++) ans[i][i] = 1;

    mat base = a;
    while (b) {
        if (b & 1) ans = mul(ans, base);
        base = mul(base, base);
        b >>= 1;
    }

    return ans;
}

void print_mat(mat& m) {
    for (size_t r = 0; r < m.size(); r++) {
        for (size_t c = 0; c < m[r].size(); c++) {
            cout << m[r][c] << " ";
        }
        cout << endl;
    }
    cout << "---------------" << endl;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    ll A, B, N, X;
    cin >> A >> B >> N >> X;

    mat K(2, vec(2, 0));
    K[0][0] = A;
    K[1][0] = B;
    K[1][1] = 1;
    // print_mat(K);

    mat Q(1, vec(2, 0));
    Q[0][0] = X;
    Q[0][1] = 1;
    // print_mat(Q);

    mat L = pow(K, N);
    // print_mat(L);

    mat res = mul(Q, L);
    // print_mat(res);

    cout << res[0][0] << "\n";

    return 0;
}
