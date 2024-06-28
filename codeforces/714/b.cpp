#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;
    vector<ll> A(N, 0ll);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    sort(A.begin(), A.end());
    int len = unique(A.begin(), A.end()) - A.begin();

    if (len <= 2) {
        cout << "YES" << endl;
        return 0;
    }

    if (len >= 4) {
        cout << "NO" << endl;
        return 0;
    }

    if (A[1] - A[0] == A[2] - A[1]) {
        cout << "YES" << endl;
        return 0;
    }

    cout << "NO" << endl;

    return 0;
}
