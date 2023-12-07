import math
n,*a=map(int,open(0).read().split())
m=a[0]
d=a[-1]-m
print(m%math.gcd(d,*(2*(x-m)for x in a))+d)