import math
a=lambda:list(map(int,input().split()))
b=a()[1:]
c=[0]+[10**18]*7
for d in a():
 e=list(c)
 for f in range(7):
  g=h=7^f
  while h:i=math.lcm(*(b[j]for j in(0,1,2)if h>>j&1));e[f|h]=min(e[f|h],c[f]+(i-d%i)%i);h=(h-1)&g
 c=e
print(c[7])