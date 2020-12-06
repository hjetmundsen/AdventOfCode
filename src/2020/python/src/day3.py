def star1():
    grid = []
    col = 0
    num_trees = 0

    for line in open('../inputs/day3.txt', 'r'):
        grid.append(line.strip())

    for row in grid:
        if row[col] == '#':
            num_trees += 1
        col = (col + 3) % len(row)

    return num_trees


def star2():
    grid = []

    for line in open('../inputs/day3.txt', 'r'):
        grid.append(line.strip())

    def solve(x, y):
        num_trees = 0
        row, col = 0, 0

        while row < len(grid):
            if grid[row][col] == '#':
                num_trees += 1
            row += y
            col = (col + x) % len(grid[0])

        return num_trees

    results = [solve(1, 1), solve(3, 1), solve(5, 1), solve(7, 1), solve(1, 2)]

    result = 1
    for i in results:
        result *= i
    return result

print('STAR 1: {}\nSTAR 2: {}'.format(star1(), star2()))
