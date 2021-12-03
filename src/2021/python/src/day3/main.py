def star1():
    inp = [x.strip() for x in open("./input", "r")]
    gamma, epsilon = "", ""

    for i in range(0, 12):
        g, e = _gamma_epsilon_helper(inp, i)
        gamma, epsilon = (gamma + g, epsilon + e)

    return int(gamma, 2) * int(epsilon, 2)


def _gamma_epsilon_helper(inp, val):
    x = len([i for i in inp if i[val] == "1"])
    return ("1", "0") if x > len(inp) // 2 else ("0", "1")


def star2():
    oxy = [x.strip() for x in open("./input", "r")]
    carb = [x.strip() for x in open("./input", "r")]
    
    for i in range(0, 12):
        oxy = _oxygen_helper(oxy, i) if len(oxy) > 1 else oxy
        carb = _carbon_helper(carb, i) if len(carb) > 1 else carb

    return int(oxy[0], 2) * int(carb[0], 2)


def _oxygen_helper(inp, val):
    x = len([i for i in inp if i[val] == "1"])
    return [x for x in inp if x[val] == "1"] if x >= len(inp) - x else [x for x in inp if x[val] == "0"]


def _carbon_helper(inp, val):
    x = len([i for i in inp if i[val] == "1"])
    return [x for x in inp if x[val] == "1"] if x < len(inp) - x else [x for x in inp if x[val] == "0"]


print("DAY 1\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(star1(), star2()))
