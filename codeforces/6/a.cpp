#include <bits/stdc++.h>
using namespace std;

#define x first
#define y second

int A[4];

int solve() {
    sort(A, A + 4);

    int ans = -1;
    do {
        int mm = *min_element(A, A + 3);
        int MM = *max_element(A, A + 3);
        int l = accumulate(A, A + 3, 0) - mm - MM;

        if (mm + l > MM) {
            ans = 0;
            break;
        }
        if (mm + l == MM) {
            ans = 1;
        }
    } while (next_permutation(A, A + 4));

    return ans;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> A[0] >> A[1] >> A[2] >> A[3];

    int ans = solve();
    if (ans == -1) cout << "IMPOSSIBLE" << endl;
    if (ans == 0) cout << "TRIANGLE" << endl;
    if (ans == 1) cout << "SEGMENT" << endl;

    return 0;
}
