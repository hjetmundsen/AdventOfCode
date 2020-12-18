def _star1():
    result = 0
    yes = set()

    for line in open("../../inputs/day6.txt", "r"):
        line = line.strip()

        if line == "":
            result += len(yes)
            yes.clear()
        else:
            for i in line:
                yes.add(i)

    return result


def _star2():
    result = 0
    responses = []

    for line in open("../../inputs/day6.txt", "r"):
        line = line.strip()

        if line == "":
            if responses:
                temp = responses[0]
                for r in responses[1:]:
                    temp = temp.intersection(r)
                result += len(temp)
                responses.clear()
        else:
            temp = set()
            for i in line:
                temp.add(i)
            responses.append(temp)

    return result


def day06():
    print("DAY 6\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
