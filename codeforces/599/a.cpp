#include <bits/stdc++.h>
using namespace std;

int main() {
    long long d1, d2, d3;
    cin >> d1 >> d2 >> d3;
    long long p[5];
    p[0] = d1 + d2 + d3;
    p[1] = d1 + d1 + d3 + d3;
    p[2] = d2 + d2 + d3 + d3;
    p[3] = d1 + d1 + d2 + d2;
    cout << *min_element(p, p + 4) << "\n";
    return 0;
}
