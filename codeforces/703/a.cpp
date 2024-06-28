#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second

typedef long long ll;
typedef pair<int, int> pii;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int win_c = 0, win_m = 0;
    int N;
    cin >> N;

    for (int i = 0; i < N; i++) {
        int m, c;
        cin >> m >> c;

        if (m > c) {
            win_m++;
            continue;
        }
        if (c > m) {
            win_c++;
            continue;
        }
    }

    if (win_m > win_c) {
        cout << "Mishka" << endl;
        return 0;
    }
    if (win_c > win_m) {
        cout << "Chris" << endl;
        return 0;
    }

    cout << "Friendship is magic!^^" << endl;

    return 0;
}
