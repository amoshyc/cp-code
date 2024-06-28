#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);
    
    ll k, r;
    cin >> k >> r;
    
    for (int i = 1; ; i++) {
        ll need = i * k;
        if (need % 10 == 0 || need % 10 == r) {
            cout << i << endl;
            return 0;
        }
    }
    
    return 0;
}