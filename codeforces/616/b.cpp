#include <bits/stdc++.h>
using namespace std;

int N, M;
int data[100][100];

int minimum[100];

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) 
        for (int j = 0; j < M; j++)
            scanf("%d", &data[i][j]);
    
    for (int i = 0; i < N; i++)
        minimum[i] = *min_element(data[i], data[i] + M);
    
    auto it = max_element(minimum, minimum + N);
    printf("%d\n", *it);
    
    return 0;
}