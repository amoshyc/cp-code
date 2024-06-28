#include <iostream>
#include <algorithm>
#include <cstdio>
#include <cstring>
#include <cstdlib>

using namespace std;

int Na, Nb;
int K, M;
int A[100000];
int B[100000];

int main() {
    scanf("%d %d", &Na, &Nb);
    scanf("%d %d", &K, &M);
    for (int i = 0; i < Na; i++)
        scanf("%d", &A[i]);
    for (int i = 0; i < Nb; i++)
        scanf("%d", &B[i]);

    // cout << A[K-1] << " " << B[Nb - M] << endl;
    if (A[K-1] < B[Nb - M])
        puts("YES");
    else
        puts("NO");

    return 0;
}
