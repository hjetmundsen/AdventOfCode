map = []

for line in open('input1.txt', 'r'):
    map.append(line.strip())


def solve(x, y):
    num_trees = 0
    row, col = 0, 0

    while row < len(map):
        if map[row][col] == '#': num_trees += 1
        row += y
        col = (col + x) % len(map[0])

    return num_trees

results = [solve(1,1), solve(3,1), solve(5,1), solve(7,1), solve(1,2)]

result = 1
for x in results: result *= x
print(result)
