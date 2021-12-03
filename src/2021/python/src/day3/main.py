def _star1():
    inp = [x.strip() for x in open("./input", "r")]

    gamma = ""
    epsilon = ""


    for i in range(0, 12):
        g, e = star1_helper(inp, i)
        gamma += g
        epsilon += e

    return int(gamma, 2) * int(epsilon, 2)


def star1_helper(inp, val):
    x = 0
    for i in inp:
        if i[val] == "1":
            x += 1

    if x > len(inp) / 2:
        return ("1", "0")
    return ("0", "1")


def _star2():
    oxy = [x.strip() for x in open("./input", "r")]
    carb = [x.strip() for x in open("./input", "r")]
    
    for i in range(0, 12):
        if len(oxy) > 1:
            oxy = oxygen(oxy, i)
        if len(carb) > 1:
            carb = carbon(carb, i)

    return int(oxy[0], 2) * int(carb[0], 2)


def oxygen(inp, val):
    num1 = 0
    num0 = 0
    for i in inp:
        if i[val] == "1":
            num1 += 1
        else:
            num0 += 1

    if num0 > num1:
        return [x for x in inp if x[val] == "0"]
    return [x for x in inp if x[val] == "1"]


def carbon(inp, val):
    num1 = 0
    num0 = 0
    for i in inp:
        if i[val] == "1":
            num1 += 1
        else:
            num0 += 1

    if num1 >= num0:
        return [x for x in inp if x[val] == "0"]
    return [x for x in inp if x[val] == "1"]


print("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
