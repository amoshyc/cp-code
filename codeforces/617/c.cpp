#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
typedef pair<ll, ll> pll; // <dis2 to circle 1, dis2 to circle 2>


const int max_n = 2000;
int N, X1, Y1, X2, Y2;
pll points[max_n + 1];

ll d2(ll a, ll b, ll c, ll d) {
    return (a - c) * (a - c) + (b - d) * (b - d);
}

int main() {
    scanf("%d %d %d %d %d", &N, &X1, &Y1, &X2, &Y2);
    for (int i = 0; i < N; i++) {
        int x, y;
        scanf("%d %d", &x, &y);
        points[i] = pll(d2(x, y, X1, Y1), d2(x, y, X2, Y2));
    }
    
    points[N] = pll(0, d2(X1, Y1, X2, Y2));
    
    sort(points, points + N + 1);
    
    // for (int i = 0; i < N; i++) 
    //     printf("%lld, %lld\n", points[i].first, points[i].second);
    
    ll ans = 1e18;
    for (int i = 0; i <= N; i++) {
        ll r1sq = points[i].first; // points[0, i] covered
        
        ll r2sq = 0;
        for (int j = i + 1; j <= N; j++)
            r2sq = max(r2sq, points[j].second);
        
        // printf("%d: %lld, %lld\n", i, r1sq, r2sq);
        
        ans = min(ans, r1sq + r2sq);
    }
    
    printf("%lld\n", ans);
    
    return 0;
}