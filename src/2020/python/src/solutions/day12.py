def _star1():
    dirs = ["N", "E", "S", "W"]
    cur_dir = "E"
    x_delta, y_delta = 0, 0
    lines = [line.strip() for line in open("../../inputs/day12.txt")]

    for line in lines:
        dir, val = line[0], int(line[1:])
        if dir == "F":
            dir = cur_dir

        if dir == "N":
            y_delta += val
        if dir == "S":
            y_delta -= val
        if dir == "E":
            x_delta += val
        if dir == "W":
            x_delta -= val
        if dir == "R":
            val = val // 90
            cur_dir = dirs[(dirs.index(cur_dir) + val) % 4]
        if dir == "L":
            val = val // 90
            cur_dir = dirs[(dirs.index(cur_dir) - val) % 4]

    return abs(x_delta) + abs(y_delta)


def _update_cur_dir(x, y):
    if x < 0 and y >= 0:
        return 0
    if x >= 0 and y >= 0:
        return 1
    if x >= 0 and y < 0:
        return 2
    return 3


def _star2():
    dirs = [(-1, 1), (1, 1), (1, -1), (-1, -1)]
    cur_dir = 1
    waypoint_x, waypoint_y = 10, 1
    ship_x, ship_y = 0, 0
    lines = [line.strip() for line in open("../../inputs/day12.txt")]

    for line in lines:
        dir, val = line[0], int(line[1:])
        if dir == "F":
            ship_x += waypoint_x * val
            ship_y += waypoint_y * val

        if dir == "N":
            waypoint_y += val
            cur_dir = _update_cur_dir(waypoint_x, waypoint_y)
        if dir == "S":
            waypoint_y -= val
            cur_dir = _update_cur_dir(waypoint_x, waypoint_y)
        if dir == "E":
            waypoint_x += val
            cur_dir = _update_cur_dir(waypoint_x, waypoint_y)
        if dir == "W":
            waypoint_x -= val
            cur_dir = _update_cur_dir(waypoint_x, waypoint_y)
        if dir == "R":
            val = val // 90
            cur_dir = (cur_dir + val) % 4
            if val == 1 or val == 3:
                waypoint_x, waypoint_y = abs(waypoint_y), abs(waypoint_x)
                waypoint_x, waypoint_y = (
                    waypoint_x * dirs[cur_dir][0],
                    waypoint_y * dirs[cur_dir][1],
                )
            if val == 2:
                waypoint_x *= -1
                waypoint_y *= -1
        if dir == "L":
            val = val // 90
            cur_dir = (cur_dir - val) % 4
            if val == 1 or val == 3:
                waypoint_x, waypoint_y = abs(waypoint_y), abs(waypoint_x)
                waypoint_x, waypoint_y = (
                    waypoint_x * dirs[cur_dir][0],
                    waypoint_y * dirs[cur_dir][1],
                )
            if val == 2:
                waypoint_x *= -1
                waypoint_y *= -1

    return abs(ship_x) + abs(ship_y)


def day12():
    print("DAY 12\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
