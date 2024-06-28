#include <bits/stdc++.h>
using namespace std;

int main() {
    int fa, ta, fb, tb;
    int h, m;
    scanf("%d %d %d %d", &fa, &ta, &fb, &tb);
    scanf("%d:%d", &h, &m);

    int a_st = h * 60 + m;
    int a_ed = a_st + ta;

    int cnt = 0;
    for (int i = 0; ; i++) {
        int b_st = 5 * 60 + i * fb;
        int b_ed = b_st + tb;

        if (b_st >= 24 * 60) break;

        if (b_ed <= a_st) continue;
        if (b_ed <= a_ed) cnt++;
        else {
            if (b_st < a_ed)
                cnt++;
        }
    }

    printf("%d\n", cnt);

    return 0;
}
