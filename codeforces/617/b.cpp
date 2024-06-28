#include <bits/stdc++.h>
using namespace std;

const int max_n = 100;
int N;
int data[max_n];

int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) {
        scanf("%d", &data[i]);
    }
    
    int first = 0;
    while (first < N && data[first] != 1) 
        first++;
    
    if (first == N) {
        puts("0");
        return 0;
    }
        
    // printf("f: %d\n", first);
    
    int start = first + 1;
    int tail;
    
    long long cnt = 1;
    while (start < N) {
        tail = start;
        while (tail < N && data[tail] == 0)
            tail++;
            
        // printf("%d, %d\n", start, tail);
        
        if (tail == N) break; // rest are all 0s
            
        // [start, tail]: 0...01
        int len = tail - start; // number of 0s
        cnt *= (len + 1);
        
        start = tail + 1;
    }
    
    printf("%lld\n", cnt);
    
    return 0;
}