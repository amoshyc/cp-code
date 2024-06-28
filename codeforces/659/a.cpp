#include <bits/stdc++.h>
using namespace std;

int main() {
    int n, a, b;
    cin >> n >> a >> b;
    int i = a - 1;
    if (b < 0) {
        i -= -b;
        while (i < 0)
            i += n;
    }
    else {
        i += b;
        i %= n;
    }

    cout << i+1 << endl;

    return 0;
}
