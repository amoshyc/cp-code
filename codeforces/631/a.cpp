#include <bits/stdc++.h>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N; cin >> N;
    int a = 0, b = 0;
    for (int i = 0; i < N; i++) {
        int inp; cin >> inp;
        a |= inp;
    }
    for (int i = 0; i < N; i++) {
        int inp; cin >> inp;
        b |= inp;
    }
    cout << a + b << endl;

    return 0;
}
