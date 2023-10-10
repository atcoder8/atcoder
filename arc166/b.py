import math
a=lambda:map(int,input().split())
_,*b=a()
c=[0]+[1<<60]*7
for d in a():
 e=7
 while e:
  e-=1;f=g=7^e
  while g:h=math.lcm(*(b[i&3]for i in(1,2,4)if g&i));c[e|g]=min(c[e|g],c[e]+(h-d%h)%h);g=~-g&f
print(c[7])