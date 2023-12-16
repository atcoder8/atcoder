from fractions import Fraction


_, decimal = input().split(".")
n = int(input())

mantissa, exp = int(decimal), 10 ** len(decimal)
org_frac = Fraction(mantissa, exp)

frac = org_frac.limit_denominator(n)
if frac > org_frac:
    diff = frac - org_frac
    small_frac = (frac - 2 * diff).limit_denominator(n)
    cand_diff = org_frac - small_frac

    if small_frac >= 0 and cand_diff == diff:
        frac = small_frac

print(frac.numerator, frac.denominator)
