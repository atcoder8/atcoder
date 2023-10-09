import math
a=lambda:list(map(int,input().split()))
b,c=a()[1:],a()
d=[10**18]*8
d[0]=0
for e in c:
    f=list(d)
    for g in range(7):
        h=i=7^g
        while i:
            j=math.lcm(*(b[k]for k in(0,1,2)if(i>>k)&1))
            f[g|i]=min(f[g|i],d[g]+(j-e%j)%j)
            i=(i-1)&h
    d=f
print(d[7])