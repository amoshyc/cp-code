#include <bits/stdc++.h>
using namespace std;

int KMP(const string P, string S) {
    int cnt = 0;
    const int Np = P.length();
    const int Ns = S.length();

    // failure function
    vector<int> F(Np, 0);
    int i = 0, j = -1; F[0] = -1;
    while (i < Np) {
        while (j >= 0 && P[i] != P[j]) j = F[j];
        i++; j++;
        F[i] = j;
    }

    i = 0, j = 0;
    while (i < Ns) {
        while (j >= 0 && S[i] != P[j]) j = F[j];
        i++; j++;
        if (j == Np) {
            cnt++;
            j = F[j];

            S[i - 1] = '#';
            i--;
        }
    }

    return cnt;
}

int main() {
    string s, p;
    cin >> s >> p;
    cout << KMP(p, s) << "\n";

    return 0;
}
