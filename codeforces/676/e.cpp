#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

const int INF = 0x3f3f3f3f;
const int H = 0; // Human
const int C = 1; // Computer

const int MAX_N = 100000;
int N, K;
int a[MAX_N + 1];

bool iszero() {
    ll s = 0;
    for (int i = N; i >= 0; i--) {
        s = s * K + a[i];

        if (abs(s) > ll(1e9))
            return false;
    }
    return (s == 0);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    cin >> N >> K;

    int this_turn = -1;
    int last_turn = -1;
    int unkown_cnt = 0;
    int known_cnt = 0;

    for (int i = 0; i <= N; i++) {
        string inp; cin >> inp;
        if (inp[0] == '?') a[i] = INF;
        else {
            known_cnt++;
            a[i] = stoi(inp);
        }
    }

    unkown_cnt = N + 1 - known_cnt;
    this_turn = (((known_cnt + 1) % 2 == 1) ? C : H);
    last_turn = (((N + 1) % 2 == 1) ? C : H);

    int win = -1;
    if (K == 0) {
        if (a[0] != INF)
            win = ((a[0] == 0) ? H : C);
        else
            win = ((this_turn == H) ? H : C);
    }
    else {
        if (unkown_cnt == 0)
            win = ((iszero()) ? H : C);
        else
            win = ((last_turn == H) ? H : C);
    }

    cout << ((win == H) ? "YES" : "NO") << endl;

    return 0;
}
