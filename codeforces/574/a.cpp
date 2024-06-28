#include <iostream>
#include <algorithm>
#include <queue>
#include <numeric>
#include <cstdio>
#include <cstdlib>
#include <cstring>

using namespace std;

int data[100];

int main() {
    priority_queue<int> pq;
    int N; scanf("%d", &N);
    int vote; scanf("%d", &vote);
    for (int i = 1; i < N; i++) {
        int v; scanf("%d", &v);
        pq.push(v);
    }

    int cnt = 0;
    while (vote <= pq.top()) {
        cnt++; vote++;
        int top = pq.top();
        pq.pop();
        pq.push(top-1);
    }

    printf("%d\n", cnt);

    return 0;
}
