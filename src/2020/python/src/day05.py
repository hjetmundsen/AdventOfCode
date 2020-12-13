def _calculate_seat_ids():
    seat_ids = []

    for line in open("../inputs/day5.txt", "r"):
        line = line.strip()
        r_min, r_max = 0, 127
        c_min, c_max = 0, 7

        for r in line[:7]:
            r_mid = r_min + ((1 + (r_max - r_min)) // 2)
            if r == "F":
                r_max = r_mid
            else:
                r_min = r_mid

        row = r_min

        for c in line[7:]:
            c_mid = c_min + ((1 + (c_max - c_min)) // 2)
            if c == "L":
                c_max = c_mid
            else:
                c_min = c_mid

        col = c_min

        seat_ids.append(row * 8 + col)

    return seat_ids


def star1():
    return max(_calculate_seat_ids())


def star2():
    seat_ids = _calculate_seat_ids()
    seat_ids.sort()
    for i in range(len(seat_ids) - 1):
        if seat_ids[i + 1] == seat_ids[i] + 2:
            return seat_ids[i] + 1


print("STAR 1: {}\nSTAR 2: {}".format(star1(), star2()))
