#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; ll c;
    cin >> N >> c;

    vector<ll> A(N, 0);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    int cnt = 1;
    for (int i = 1; i < N; i++) {
        if (A[i] - A[i - 1] > c) {
            cnt = 1;
        }
        else {
            cnt++;
        }
    }

    cout << cnt << endl;

    return 0;
}
