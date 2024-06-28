#include <bits/stdc++.h>
using namespace std;

int main() {
    int a, b;
    cin >> a >> b;

    if (b > a)
        swap(a, b);
    // a > b

    cout << min(a, b) << " " << (a - min(a, b)) / 2 << endl;

    return 0;
}
