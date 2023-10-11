import math
a=lambda:map(int,input().split())
_,*b=a()
c=[0]+[2**60]*7
for d in a():
 e=7
 while e:
  e-=1;f=7
  while f:g=math.lcm(*(b[i&3]for i in(1,2,4)if f&i));c[e|f]=min(c[e|f],c[e]+(g-d%g)%g);f-=1
print(c[7])