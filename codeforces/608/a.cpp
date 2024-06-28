#include <bits/stdc++.h>
using namespace std;

int main() {
    int N, S;
    scanf("%d %d", &N, &S);

    vector<int> last_time(S + 1, 0);
    for (int i = 0; i < N; i++) {
        int f, t; scanf("%d %d", &f, &t);
        last_time[f] = max(last_time[f], t);
    }

    int cur_time = 0;
    for (int f = S; f >= 1; f--) {
        if (last_time[f] > cur_time) // passenger
            cur_time = last_time[f];
        cur_time++; // move to f-1
    }
    printf("%d\n", cur_time);

    return 0;
}
