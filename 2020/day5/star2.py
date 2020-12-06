def solve():
    results = []

    for line in open('input1.txt', 'r'):
        line = line.strip()
        r_min, r_max = 0, 127
        c_min, c_max = 0, 7

        for r in line[:7]:
            r_mid = r_min + ((1 + (r_max - r_min)) // 2)
            if r == 'F': r_max = r_mid
            else: r_min = r_mid

        row = r_min

        for c in line[7:]:
            c_mid = c_min + ((1 + (c_max - c_min)) // 2)
            if c == 'L': c_max = c_mid
            else: c_min = c_mid

        col = c_min

        results.append(row * 8 + col)

    results.sort()
    for i in range(len(results)-1):
        if results[i+1] == results[i] + 2: return results[i] + 1

print(solve())
