#include <algorithm>
#include <atcoder/modint>
#include <iostream>
#include <tuple>
#include <vector>
using namespace std;
using mint = atcoder::modint;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int TC;
    cin >> TC;
    while (TC--) {
        int N, S, K;
        cin >> N >> S >> K;

        int g = __gcd(K, N);
        if (S % g != 0) {
            cout << "-1\n";
        } else {
            mint::set_mod(N / g);
            mint x = mint(K / g).inv() * mint(-S / g);
            cout << x.val() << endl;
        }
    }

    return 0;
}