#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;

int N, X;
vector<int> A;
map<int, int> m;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> X;

    A = vector<int>(N);
    for (int i = 0; i < N; i++)
        cin >> A[i];

    for (int i : A) {
        m[i]++;
    }

    ll cnt = 0;
    for (int i : A) {
        int c = X ^ i;
        if (c == i)
            cnt += m[c] - 1;
        else
            cnt += m[c];
    }

    cnt /= 2;

    cout << cnt << endl;

    return 0;
}
