map = []
col = 0
num_trees = 0

for line in open('input1.txt', 'r'):
    map.append(line.strip())

for row in map:
    if row[col] == '#': num_trees += 1
    col = (col + 3) % len(row)

print(num_trees)
