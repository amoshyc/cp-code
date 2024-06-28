#include <iostream>
#include <algorithm>
#include <queue>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>

using namespace std;

int N;
int data[100000];

int gcd(int a, int b) {
    while (b) {
        // a, b = b, a % b
        int temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

bool possible() {
    int g = gcd(data[0], data[1]);
    for (int i = 2; i < N; i++)
        g = gcd(g, data[i]);

    for (int i = 0; i < N; i++) {
        data[i] /= g;

        while (data[i] % 2 == 0)
            data[i] /= 2;
        while (data[i] % 3 == 0)
            data[i] /= 3;

        if (data[i] != 1)
            return false;
    }

    return true;
}


int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++)
        scanf("%d", &data[i]);

    if (possible())
        puts("Yes");
    else
        puts("No");

    return 0;
}
