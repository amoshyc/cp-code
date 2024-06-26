#include <bits/stdc++.h>
using namespace std;

typedef unsigned int ui;
typedef long double ldb;
const ldb pi = atan2(0, -1);

struct Complex {
    ldb real, imag;
    Complex(): real(0.0), imag(0.0) {;}
    Complex(ldb a, ldb b) : real(a), imag(b) {;}
    Complex conj() const {
        return Complex(real, -imag);
    }
    Complex operator + (const Complex& c) const {
        return Complex(real + c.real, imag + c.imag);
    }
    Complex operator - (const Complex& c) const {
        return Complex(real - c.real, imag - c.imag);
    }
    Complex operator * (const Complex& c) const {
        return Complex(real*c.real - imag*c.imag, real*c.imag + imag*c.real);
    }
    Complex operator / (ldb x) const {
        return Complex(real / x, imag / x);
    }
    Complex operator / (const Complex& c) const {
        return *this * c.conj() / (c.real * c.real + c.imag * c.imag);
    }
};

inline ui rev_bit(ui x, int len){
	x = ((x & 0x55555555u) << 1)  | ((x & 0xAAAAAAAAu) >> 1);
	x = ((x & 0x33333333u) << 2)  | ((x & 0xCCCCCCCCu) >> 2);
	x = ((x & 0x0F0F0F0Fu) << 4)  | ((x & 0xF0F0F0F0u) >> 4);
	x = ((x & 0x00FF00FFu) << 8)  | ((x & 0xFF00FF00u) >> 8);
	x = ((x & 0x0000FFFFu) << 16) | ((x & 0xFFFF0000u) >> 16);
	return x >> (32 - len);
}

void fft(vector<Complex>& a, int flag = +1) { // flag = -1 if ifft else +1
    int n = a.size(); // n should be power of 2

    int len = __builtin_ctz(n);
    for (int i = 0; i < n; i++) {
        int rev = rev_bit(i, len);

        if (i < rev)
            swap(a[i], a[rev]);
    }

    for (int m = 2; m <= n; m <<= 1) { // width of each item
        auto wm = Complex(cos(2 * pi / m), flag * sin(2 * pi / m));
        for (int k = 0; k < n; k += m) { // start idx of each item
            auto w = Complex(1, 0);
            for (int j = 0; j < m / 2; j++) { // iterate half
                Complex t = w * a[k + j + m / 2];
                Complex u = a[k + j];
                a[k + j] = u + t;
                a[k + j + m / 2] = u - t;
                w = w * wm;
            }
        }
    }

    if (flag == -1) { // if it's ifft
        for (int i = 0; i < n; i++)
            a[i].real /= n;
    }
}

vector<int> mul(const vector<int>& a, const vector<int>& b) {
    int n = int(a.size()) + int(b.size()) - 1;
    int nn = 1;
    while (nn < n)
        nn <<= 1;

    vector<Complex> fa(nn, Complex(0, 0));
    vector<Complex> fb(nn, Complex(0, 0));
    for (int i = 0; i < int(a.size()); i++)
        fa[i] = Complex(a[i], 0);
    for (int i = 0; i < int(b.size()); i++)
        fb[i] = Complex(b[i], 0);

    fft(fa, +1);
    fft(fb, +1);
    for (int i = 0; i < nn; i++) {
        fa[i] = fa[i] * fb[i];
    }
    fft(fa, -1);

    vector<int> c;
    for(int i = 0; i < nn; i++) {
        int val = int(fa[i].real + 0.5);
        if(val) {
            while(int(c.size()) <= i)
                c.push_back(0);
            c[i] = 1;
        }
    }

    return c;
}

int main() {
    int N, K;
    scanf("%d %d", &N, &K);

    vector<int> a;
    for (int i = 0; i < N; i++) {
        int inp; scanf("%d", &inp);
        while (int(a.size()) <= inp)
            a.push_back(0);
        a[inp] = 1;
    }

    // a^k
    vector<int> ans(1, 1); // unit polynomial
    vector<int> base(a);
    while (K) {
        if (K & 1) ans = mul(ans, base);
        K >>= 1;
        base = mul(base, base);
    }

    for (int i = 0; i < int(ans.size()); i++) {
        if (ans[i] > 0) {
            printf("%d ", i);
        }
    }
    puts("");

    return 0;
} 
