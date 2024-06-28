#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    vector<int> A(N, 0);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    sort(A.begin(), A.end());

    int Q;
    cin >> Q;
    for (int i = 0; i < Q; i++) {
        int m; cin >> m;
        cout << int(upper_bound(A.begin(), A.end(), m) - A.begin()) << "\n";
    }

    return 0;
}
