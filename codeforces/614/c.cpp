#include <bits/stdc++.h>
using namespace std;

const int max_n = 100000;
const double eps = 1e-9;
const double pi = atan2(0, -1);

int xs[max_n];
int ys[max_n];

double d2(double x1, double y1, double x2, double y2) {
    return (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
}

double d2_seg(double x, double y, double x1, double y1, double x2, double y2) {
    double a = (x2 - x1);
    double b = (y2 - y1);
    double t = (-1.0) * ((a * (x1 - x) + b * (y1 - y)) / (a * a + b * b));
    
    if (fabs(t - 0.0) < eps) { // t = 0.0
        return d2(x, y, x1, y1);
    }
    if (fabs(t - 1.0) < eps) { // t = 1.0
        return d2(x, y, x2, y2);
    }
    // 0.0 < t < 1.0
    if (t - 0.0 > -eps && t - 1.0 < eps) {
        double qx = x1 + a * t, qy = y1 + b * t;
        // printf("Q: %lf %lf\n", qx, qy);
        return d2(x, y, qx, qy);
    }
    
    return min(d2(x, y, x1, y1), d2(x, y, x2, y2));
}

int main() {
    int N, px, py;
    scanf("%d %d %d", &N, &px, &py);
    for (int i = 0; i < N; i++) {
        scanf("%d %d", &xs[i], &ys[i]);
    }
    
    double farest_d2 = d2(px, py, xs[0], ys[0]);
    double nearest_d2 = d2_seg(px, py, xs[N-1], ys[N-1], xs[0], ys[0]);
    for (int i = 0; i < N; i++) {
        double dis2 = d2(px, py, xs[i], ys[i]);
        if (dis2 > farest_d2)
            farest_d2 = dis2;
    }
    
    for (int i = 0; i + 1 < N; i++) {
        // printf("(%d, %d), (%d, %d)\n", xs[i], ys[i], xs[i+1], ys[i+1]);
        double dis2_seg = d2_seg(px, py, xs[i], ys[i], xs[i + 1], ys[i + 1]);
        // printf("%lf\n", dis2_seg);
        if (dis2_seg < nearest_d2)
            nearest_d2 = dis2_seg;
    }
    
    // printf("%lf, %lf\n", nearest_d2, farest_d2);
    
    double area = pi * (farest_d2 - nearest_d2);
    printf("%.10lf\n", area);
    
    return 0;
}