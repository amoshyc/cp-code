#include <bits/stdc++.h>
using namespace std;

int main() {
    string a, b;
    cin >> a >> b;

    int len_a = a.length(), len_b = b.length(), len_delta = len_b - len_a + 1;
    // S0[i] = the number of 0 in b[0, i)
    // S1[i] = the number of 1 in b[0, i)
    int S0[len_b + 1]; S0[0] = 0;
    int S1[len_b + 1]; S1[0] = 0;
    for (int i = 1; i <= len_b; i++) {
        S0[i] = S0[i-1] + (b[i-1] == '0');
        S1[i] = S1[i-1] + (b[i-1] == '1');
    }

    long long ans = 0;
    for (int i = 0; i < len_a; i++) {
        if (a[i] == '0') // the number of 1 in b[i+1, i + len_delta)
            ans += S1[i + len_delta] - S1[i];
        else // the number of 0 in b[i+1, i + len_delta)
            ans += S0[i + len_delta] - S0[i];
    }

    cout << ans << "\n";

    return 0;
}
