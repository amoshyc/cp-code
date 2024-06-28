// #include <stdio.h>
// #include <stdlib.h>
#include <cstdio>
#include <cstdlib>
#include <algorithm>
using namespace std;

int n,m;
int i,j,p;
int a[200000];
int b[200000];

int cmp(const void *a,const void *b)
{
    return *(int*)a-*(int*)b;
}

int bs(int k)
{
    int l=0,r=n-1;
    while(l<=r)
    {
        int mid=(l+r)/2;
        if(a[mid]<=k)
        l=mid+1;
        else if(a[mid]>k)
        r=mid-1;
        else
        r=mid-1;
    }
    return l;
    
    // return upper_bound(a, a + n, k) - a;
}


int main()
{
    scanf("%d%d",&n,&m);
    for(i=0;i<n;i++)
        scanf("%d",&a[i]);
    
    // qsort(a,n,sizeof(int),cmp);
    sort(a, a + n);
    
    for(j=0;j<m;j++)
        scanf("%d",&b[j]);
        
    for(j=0;j<m;j++)
        printf("%d ",bs(b[j]));
    
    return 0;
}