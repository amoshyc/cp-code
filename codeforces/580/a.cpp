#include <bits/stdc++.h>
using namespace std;

int N;
int data[100000];

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++)
        scanf("%d", &data[i]);

    int max_len = 1;
    int len = 1;
    for (int i = 1; i < N; i++) {
        if (data[i] >= data[i-1]) {
            len++;
            max_len = max(max_len, len);
        }
        else {
            len = 1;
        }
    }
    printf("%d\n", max_len);

    return 0;
}
