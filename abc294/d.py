n, q = map(int, input().split())
events = [list(map(int, input().split())) for _ in range(q)]

used = [False] * n
min_wait_idx = 0

for event in events:
    if event[0] == 2:
        used[event[1] - 1] = True
    elif event[0] == 3:
        while used[min_wait_idx]:
            min_wait_idx += 1
        print(min_wait_idx + 1)
