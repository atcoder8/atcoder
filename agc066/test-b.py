def calc_sum_digit(n: int) -> int:
    s = 0
    while n != 0:
        s += n % 10
        n //= 10

    return s


def count_inc_len(x: int) -> int:
    inc_len = 0
    while True:
        next_x = 5 * x

        if calc_sum_digit(next_x) <= calc_sum_digit(x):
            break

        inc_len += 1
        x = next_x

    return inc_len


if __name__ == "__main__":
    max_cnt = 0
    for x in range(10**8):
        cnt = count_inc_len(x)

        if cnt > max_cnt:
            print(f"{x}: {cnt}")
            max_cnt = cnt
