def _star1():
    instructions = [line for line in open("../../inputs/day8.txt", "r")]
    visited = set()
    i, acc = 0, 0
    signs = {"+": 1, "-": -1}

    while True:
        if i in visited:
            return acc

        visited.add(i)

        code, val = instructions[i].split(" ")

        if code == "nop":
            i += 1
            continue
        if code == "acc":
            acc += int(val[1:]) * signs[val[0]]
            i += 1
            continue
        else:
            i += int(val[1:]) * signs[val[0]]
            continue


def _star2():
    return 0


def day08():
    print("DAY 8\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
