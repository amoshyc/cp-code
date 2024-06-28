#include <bits/stdc++.h>
using namespace std;

char s[300000 + 1];
unsigned long long cnt = 0;

int main() {
    scanf("%s", s);
    int len = strlen(s);

    for (int i = 0; i < len; i++)
        if (s[i] == '0' || s[i] == '4' || s[i] == '8')
            cnt++;

    for (int i = 1; i < len; i++) {
        int val = (s[i - 1] - '0') * 10 - (s[i] - '0');
        if (val % 4 == 0) {
            cnt++; // itself
            cnt += i - 1;
        }
    }

    printf("%llu\n", cnt);

    return 0;
}
