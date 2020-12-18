def _star1():
    result = 0

    for line in open("../../inputs/day2.txt", "r"):
        ct, ch, pswd = line.split(" ")
        ct = ct.split("-")
        ch = ch[0]
        count = pswd.count(ch)
        if int(ct[0]) <= count <= int(ct[1]):
            result += 1

    return result


def _star2():
    result = 0

    for line in open("../../inputs/day2.txt", "r"):
        ct, ch, pswd = line.split(" ")
        ct = ct.split("-")
        ch = ch[0]
        if (pswd[int(ct[0]) - 1] == ch) ^ (pswd[int(ct[1]) - 1] == ch):
            result += 1

    return result


def day02():
    print("DAY 2\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
