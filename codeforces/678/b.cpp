#include <bits/stdc++.h>
using namespace std;

bool is_leap_year(int y) {
    if (y % 400 == 0) return true;
    if (y % 100 == 0) return false;
    if (y % 4 == 0) return true;
    return false;
}

int year_cnt(int y) {
    if (is_leap_year(y)) return 366;
    return 365;
}

int main() {
    int Y; cin >> Y;

    int delta = year_cnt(Y) % 7;

    int y = Y + 1;
    while (delta % 7 != 0 || is_leap_year(y) != is_leap_year(Y)) {
        delta = (delta + year_cnt(y)) % 7;
        y++;
    }

    cout << y << endl;

    return 0;
}
