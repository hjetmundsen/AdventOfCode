def _star1():
    inp = [x for x in open("./input", "r")]
    h, d = 0, 0

    for i in inp:
        di, v = i.split(" ")
        if di == "forward":
            h += int(v)
        elif di == "down":
            d += int(v)
        elif di == "up":
            d -= int(v)

    return h * d


def _star2():
    inp = [x for x in open("./input", "r")]
    h, d = 0, 0
    aim = 0

    for i in inp:
        di, v = i.split(" ")
        if di == "forward":
            h += int(v)
            d += aim * int(v) 
        elif di == "down":
            aim += int(v)
        elif di == "up":
            aim -= int(v)

    return h * d


print("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
