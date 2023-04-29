from pprint import pprint

from functional import seq

with open("times.txt") as f:
    lines = list(f)

A = (
    seq(lines)
    .map(lambda x: x.split()[2])
    .map(lambda x: x.split(":"))
    .map(lambda x: float(x[0]) / 60 + float(x[1]) / 60)
    .sum()
)
pprint(A)
