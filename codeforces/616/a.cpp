#include <bits/stdc++.h>
using namespace std;

const int max_n = 1000000 + 1;
char A[max_n];
char B[max_n];

int main() {
    scanf("%s %s", A, B);
    
    char* a = &A[0];
    char* b = &B[0];
    for (; *a == '0'; a++);
    for (; *b == '0'; b++);
    
    int len1 = strlen(a);
    int len2 = strlen(b);
    
    if (len1 < len2) {
        puts("<");
        return 0;
    }
    if (len1 > len2) {
        puts(">");
        return 0;
    }
    
    int flag = strcmp(a, b);
    if (flag < 0) {
        puts("<");
        return 0;
    }
    if (flag > 0) {
        puts(">");
        return 0;
    }
    
    puts("=");
    
    
    return 0;
}