def _star1():
    inp = [int(x) for x in open("./input", "r")]
    prev, total = inp[0], 0
    for x in inp[1:]:
        if x > prev:
            total += 1
        prev = x

    return total


def _star2():
    inp = [int(x) for x in open("./input", "r")]
    prev_window, total = inp[0] + inp[1] + inp[2], 0

    for i in range(3, len(inp)):
        cur_window = inp[i-2] + inp[i-1] + inp[i]
        if cur_window > prev_window:
            total += 1
        prev_window = cur_window

    print(total)


print("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
