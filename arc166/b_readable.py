import math


# 1行読み込んで整数のイテレータを返す関数を定義
input_integers = lambda: map(int, input().split())

# nを読み捨ててa,b,cをリストabcとして取得
_, *abc = input_integers()

# dp[bit]をbitに対応するabcの各要素の倍数が数列Aに含まれるようにするために必要な操作回数の最小値とする
# dp = [0, INF, INF, INF, INF, INF, INF, INF] (INFは十分大きな値) で初期化
dp = [0] + [2**60] * 7

# dpを各A_iについてinlineで更新
for a in input_integers():
    # dp[bit]を遷移元として配るDPを行う (whileブロックの先頭で1を引くので、使用されるbitの値は6から0)
    # bitを大きい順に見ていくので、遷移元の値が既に更新されているということは起こらない
    bit = 7
    while bit:
        bit -= 1

        # bitとのビット論理和をとる値
        add_bit = 7
        while add_bit:
            # add_bitに対応するabcの要素の最小公倍数を求める
            # i&3で1,2,4をそれぞれ1,2,0に変換してリストabcの要素を参照している
            lcm = math.lcm(*(abc[i & 3] for i in (1, 2, 4) if add_bit & i))

            # dp[bit]にA_iをlcmの倍数にするために必要な操作回数の最小値を加算した値をdp[bit|add_bit]に配る
            dp[bit | add_bit] = min(dp[bit | add_bit], dp[bit] + (lcm - a % lcm) % lcm)

            # add_bitを更新
            add_bit -= 1

# 数列Aにa,b,c全ての倍数が存在するようにするために必要な操作回数の最小値
print(dp[7])
