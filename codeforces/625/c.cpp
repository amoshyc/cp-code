#include <bits/stdc++.h>
using namespace std;

const int max_n = 500;

int n, k;
int a[max_n][max_n];

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> n >> k; k--;
    int idx = n * n;

    for (int r = n - 1; r >= 0; r--) {
        for (int c = n - 1; c >= k; c--) {
            a[r][c] = idx--;
        }
    }
    for (int r = n - 1; r >= 0; r--) {
        for (int c = k - 1; c >= 0; c--) {
            a[r][c] = idx--;
        }
    }

    int s = 0;
    for (int r = 0; r < n; r++)
        s += a[r][k];

    cout << s << "\n";
    for (int r = 0; r < n; r++) {
        for (int c = 0; c < n; c++)
            cout << a[r][c] << " ";
        cout << "\n";
    }

    return 0;
}
