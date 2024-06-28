#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, p, q;
    string s;
    cin >> N >> p >> q;
    cin >> s;
    N = s.length();

    if (p + q == N) {
        cout << "2\n";
        cout << s.substr(0, p) << "\n";
        cout << s.substr(p, q) << "\n";
    }
    else if (N % p == 0) {
        cout << N / p << "\n";
        for (int idx = 0; idx < N; idx += p)
            cout << s.substr(idx, p) << "\n";
    }
    else if (N % q == 0) {
        cout << N / q << "\n";
        for (int idx = 0; idx < N; idx += q)
            cout << s.substr(idx, q) << "\n";
    }
    else {
        for (int a = 0; a < N; a++) {
            for (int b = 0; b < N; b++) {
                if (a * p + b * q == N) {
                    printf("%d\n", a + b);
                    int idx = 0;
                    for (int i = 0; i < a; i++) {
                        cout << s.substr(idx, p) << "\n";
                        idx += p;
                    }
                    for (int i = 0; i < b; i++) {
                        cout << s.substr(idx, q) << "\n";
                        idx += q;
                    }
                    return 0;
                }
            }
        }

        cout << "-1\n";
    }

    return 0;
}
