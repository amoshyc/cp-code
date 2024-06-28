#include <stdlib.h>
#include <stdio.h>
#include <math.h>

long long int l,r,k;
int i=0,j=0;
long long int r1,r2;
int x=0;
int y=0;
long long int pf(long long int k,long long int i)
{
    long long int temp=1;
    if(i==0)
    return 1;
    else
    {
        for(j=0;j<i;j++)
        {
            temp=k*temp;
        }
        k=temp;
        //printf("k:%lld\n",k);
        return k;
    }
}

int main()
{
    
    scanf("%lld",&l);
    scanf("%lld",&r);
    scanf("%lld",&k);
    while(i<=r)
    {
        if(pf(k,i)>=l)
        {
            r1=i;
            x=1;
            //printf("%lld\n",pf(k,i));
            //printf("r1:%lld\n",r1);
            break;
        }
        i++;
    }
    j = 0;
    while(j<=r)
    {
        if(pf(k,j)>r/k)
        {
            y=1;
            //printf("j=%d\n",j);
            //printf("%lld\n",pf(k,j));
            r2=j;
            //printf("r2:%lld\n",r2);
            break;
        }
        j++;
    }
    //printf("x:%d\n",x);
    if(r1>r2&&x==0&&y==0)
        printf("-1");
    else if(r1==r2)
    {
        if(pf(k,r1)>=l&&pf(k,r2)<=r)
        {
            printf("%lld",pf(k,r1));
        }
        else
            printf("-1");
    }
    else if (r2 < r1) {
        puts("-1");
    }
    else
    {
        for(i=r1;i<=r2;i++)
        {
          printf("%lld ",pf(k,i));
        }
    }
    
    return 0;
}