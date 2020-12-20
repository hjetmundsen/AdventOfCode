directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]


def _is_in_bounds(row: int, col: int, row_edge: int, col_edge: int):
    return row >= 0 and row < row_edge and col >= 0 and col < col_edge


def _adjacent_occupied_count(grid: list[str], row: int, col: int) -> int:
    result = 0
    for direction in directions:
        if (
            _is_in_bounds(
                row + direction[0], col + direction[1], len(grid), len(grid[0])
            )
            and grid[row + direction[0]][col + direction[1]] == "#"
        ):
            result += 1

    return result


def _first_visible_adjacent_occupied_count(grid: list[str], row: int, col: int) -> int:
    result = 0
    for direction in directions:
        vis_row = row + direction[0]
        vis_col = col + direction[1]
        while _is_in_bounds(vis_row, vis_col, len(grid), len(grid[0])):
            if grid[vis_row][vis_col] == "L":
                break
            if grid[vis_row][vis_col] == "#":
                result += 1
                break
            vis_row += direction[0]
            vis_col += direction[1]

    return result


def _solution(first_visible: bool, num_occupied_to_change: int) -> int:
    grid = [[c for c in line.strip()] for line in open("../../inputs/day11.txt")]
    changes = [[False for _ in range(len(grid[0]))] for _ in range(len(grid))]
    state_change = True

    while state_change:
        state_change = False

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if grid[i][j] == ".":
                    continue

                adjacent_occupied = (
                    _first_visible_adjacent_occupied_count(grid, i, j)
                    if first_visible
                    else _adjacent_occupied_count(grid, i, j)
                )
                if adjacent_occupied >= num_occupied_to_change and grid[i][j] == "#":
                    changes[i][j] = True
                elif adjacent_occupied == 0 and grid[i][j] == "L":
                    changes[i][j] = True
                else:
                    changes[i][j] = False

        for i in range(len(grid)):
            for j in range(len(grid[0])):
                if changes[i][j]:
                    state_change = True
                    grid[i][j] = "#" if grid[i][j] == "L" else "L"

    result = 0
    for row in grid:
        for col in row:
            if col == "#":
                result += 1

    return result


def day11():
    print(
        "DAY 11\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(
            _solution(False, 4), _solution(True, 5)
        )
    )
