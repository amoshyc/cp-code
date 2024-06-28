#include <bits/stdc++.h>
using namespace std;

#define st first
#define nd second
typedef pair<int, int> pii;
typedef long long ll;

const int MAX_N = 100000;
const int MAX_M = 100000;
int N, M;
int A[MAX_N];
int B[MAX_M];

bool C(ll x) {
    vector<bool> occur(M, false);
    vector<pii> exam_day;
    for (int i = x - 1; i >= 0; i--) {
        if (A[i] != -1 && !occur[A[i]]) {
            occur[A[i]] = true;
            exam_day.push_back(pii(A[i], i)); // <type, day>
        }
    }
    
    for (int i = 0; i < M; i++)
        if (!occur[i]) 
            return false;
    
    reverse(exam_day.begin(), exam_day.end());
    int cnt = 0;
    int idx = 0;
    for (int i = 0; i < x; i++) {
        if (i == exam_day[idx].nd) {
            if (cnt < B[exam_day[idx].st]) {
                return false;
            }
            cnt -= B[exam_day[idx].st];
            idx++;
        }
        else {
            cnt++;
        }
    }
    
    return true;
}

int solve() {
    int lb = 1, ub = N;
    // 0 0 0 1 1 1
    if (C(lb)) return lb;
    if (!C(ub)) return -1;
    
    while (ub - lb > 1) {
        int m = (lb + ub) / 2;
        if (C(m)) ub = m;
        else lb = m;
    }
    
    return ub;
}

int main() {
    scanf("%d %d", &N, &M);
    for (int i = 0; i < N; i++) {
        scanf("%d", &A[i]);
        A[i]--;
    }
    for (int i = 0; i < M; i++) {
        scanf("%d", &B[i]);
    }
    
    printf("%d\n", solve());
    
    return 0;
}