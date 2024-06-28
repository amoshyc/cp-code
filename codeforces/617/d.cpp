#include <bits/stdc++.h>
using namespace std;

int x[3];
int y[3];

bool f(int i, int j, int k) {
    if (x[j] == x[k] && !(min(y[j], y[k]) < y[i] && y[i] < max(y[j], y[k])))
        return true;
    if (y[j] == y[k] && !(min(x[j], x[k]) < x[i] && x[i] < max(x[j], x[k])))
        return true;
    return false;
}

int main() {
    for (int i = 0; i < 3; i++)
        scanf("%d %d", &x[i], &y[i]);

    if ((x[0] == x[1] && x[1] == x[2]) || (y[0] == y[1] && y[1] == y[2]))
        puts("1");
    else if (f(0, 1, 2) || f(1, 0, 2) || f(2, 0, 1))
        puts("2");
    else
        puts("3");

    return 0;
}
