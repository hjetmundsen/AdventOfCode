def solve():
    s = set()

    for line in [int(x) for x in open('input1.txt', 'r')]:
        if (2020 - line) in s: return (line * (2020 - line))
        s.add(line)

print(solve())
