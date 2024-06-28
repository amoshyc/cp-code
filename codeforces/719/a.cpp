#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    vector<int> a(N, 0);
    for (int i = 0; i < N; i++) {
        cin >> a[i];
    }

    vector<int> b;
    while (int(b.size()) < 4 * N || b.size() < 100) {
        for (int i = 0; i < 15; i++)
            b.push_back(i);
        for (int i = 15; i > 0; i--)
            b.push_back(i);
    }

    // for (int i : b) {
    //     cout << i << " ";
    // }
    // cout << endl;

    int nxt = -1;
    for (int i = 0; i + N < int(b.size()); i++) {
        if (equal(a.begin(), a.end(), b.begin() + i)) {
            if (nxt == -1) {
                nxt = b[i + N];
                continue;
            }
            if (b[i + N] != nxt) {
                cout << "-1" << endl;
                return 0;
            }
        }
    }

    if (nxt > a.back())
        cout << "UP" << endl;
    else
        cout << "DOWN" << endl;

    return 0;
}
