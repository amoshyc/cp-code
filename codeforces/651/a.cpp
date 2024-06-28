#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    int a, b;
    cin >> a >> b;
    
    if (a == 1 && b == 1) {
        cout << "0\n";
        return 0;
    }
    
    int t = 0;
    while (a > 0 && b > 0) {
        if (a < b) {
            a += 1;
            b -= 2;
        }
        else {
            a -= 2;
            b += 1;
        }
        
        t++;
    }
    cout << t << endl;
    
    return 0;
}