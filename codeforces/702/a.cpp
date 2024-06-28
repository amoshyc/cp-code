#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
// const int INF = 0x3f3f3f3f;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N;
    cin >> N;

    vector<int> A(N, 0);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    int ans = -1;
    int s = 0;
    while (s < N) {
        int t = s + 1;
        while (t < N && A[t] > A[t - 1]) {
            t++;
        }

        ans = max(ans, t - s);

        if (t == N) {
            break;
        }

        s = t;
    }

    cout << ans << endl;

    return 0;
}
