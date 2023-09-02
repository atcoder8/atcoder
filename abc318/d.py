n = int(input())
aa = list(map(lambda x: int(x) - 1, input().split()))

left = [0] * n
right = [0] * n
for a in aa:
    right[a] += 1

ans = 0
comb = 0
for a in aa:
    right[a] -= 1
    comb -= left[a]
    ans += comb - left[a] * right[a]
    left[a] += 1
    comb += right[a]

print(ans)
