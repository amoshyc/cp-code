#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<int, int> pii;
#define st first
#define nd second;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(0);

    int N, M;
    scanf("%d %d", &N, &M);

    bool ok = true;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            char inp[5];
            scanf("%s", inp);

            if (inp[0] == 'C' || inp[0] == 'M' || inp[0] == 'Y')
                ok = false;
        }
    }

    puts((ok) ? "#Black&White" : "#Color");

    return 0;
}
