#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int N, M;
ll add;
vector<ll> A;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    cout.tie(0);

    cin >> N >> M;
    A = vector<ll> (N, 0);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    while (M--) {
        int cmd; cin >> cmd;
        if (cmd == 1) {
            int idx, x;
            cin >> idx >> x;
            idx--;

            A[idx] += -(A[idx] + add); // change to zero
            A[idx] += x;
        }
        else if (cmd == 2) {
            int delta; cin >> delta;
            add += delta;
        }
        else {
            int idx; cin >> idx; idx--;
            cout << (A[idx] + add) << endl;
        }
    }

    return 0;
}
