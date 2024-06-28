#include <bits/stdc++.h>
using namespace std;

const int rem2[4] = {1, 2, 4, 3};
const int rem3[4] = {1, 3, 4, 2};
const int rem4[2] = {1, 4};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    string inp;
    cin >> inp;

    const int N = inp.length();
    int mod_2 = 0, mod_4 = 0;
    if (N >= 2) {
        mod_2 = (((inp[N - 1] - '0')) % 2);
        mod_4 = (((inp[N - 2] - '0') * 10 + (inp[N - 1] - '0')) % 4);
    }
    else {
        mod_2 = (((inp[N - 1] - '0')) % 2);
        mod_4 = (((inp[N - 1] - '0')) % 4);
    }

    int r1 = 1;
    int r2 = rem2[mod_4];
    int r3 = rem3[mod_4];
    int r4 = rem4[mod_2];

    cout << (r1 + r2 + r3 + r4) % 5 << "\n";

    return 0;
}
