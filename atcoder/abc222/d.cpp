#include <algorithm>
#include <atcoder/modint>
#include <iostream>
#include <vector>
using namespace std;

using mint = atcoder::modint998244353;

int main() {
    int N;
    cin >> N;

    vector<int> A(N);
    vector<int> B(N);
    for (int i = 0; i < N; i++) {
        cin >> A[i];
    }
    for (int i = 0; i < N; i++) {
        cin >> B[i];
    }

    vector<mint> prev(3333, 0);
    for (int v = A[0]; v <= B[0]; v++) {
        prev[v] += 1;
    }

    for (int i = 1; i < N; i++) {
        vector<mint> curr(3333, 0);

        mint pref = 0;
        for (int v = 0; v < A[i]; v++) {
            pref += prev[v];
        }
        for (int v = A[i]; v <= B[i]; v++) {
            pref += prev[v];
            curr[v] = pref;
        }

        prev = curr;
    }

    mint ans = 0;
    for (int v = A.back(); v <= B.back(); v++) {
        ans += prev[v];
    }

    cout << ans.val() << endl;

    return 0;
}