#include <bits/stdc++.h>
using namespace std;

const int max_n = 100;
int N;
int x[max_n];

bool C(int p) {
    if (p == 0) return false;
    
    priority_queue<int> max_pq_s;
    for (int i = 0; i < p; i++)
        max_pq_s.push(x[i]);
    
    for (int i = p; i < N; i++) {
        int top = max_pq_s.top();
        max_pq_s.pop();
        if (top > 0)
            max_pq_s.push(min(x[i], top - 1));
        else
            return false;
    }
    
    return true;
}

int solve() {
    sort(x, x + N, greater<int>()); // --
    
    // C(p): is it possible to construct (>= p) piles
    // 0 0 0 1 1 1
    int lb = 0, ub = N;
    while (ub - lb > 1) {
        int mid = (lb + ub) / 2;
        if (C(mid)) ub = mid;
        else lb = mid;
    }
    
    return ub;
}


int main() {
    scanf("%d", &N);
    for (int i = 0; i < N; i++) 
        scanf("%d", &x[i]);
    
    printf("%d\n", solve());
    
    return 0;
}