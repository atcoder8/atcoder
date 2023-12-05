n,l,*a=map(int,open(0).read().split())
d=l
b=n-1
for t in range(n):
 while b>t and a[t]+a[b]>l:b-=1
 for i in 0,1:d=min(d,abs(a[t]+a[(b+i)%n]-l))
print(l+d<<1)