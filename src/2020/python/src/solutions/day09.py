def _sum_pair_exists(window: set[int], num: int) -> bool:
    for w in window:
        if (num - w) in window:
            return True

    return False


def _star1():
    xmas = [int(line.strip()) for line in open("../../inputs/day9.txt")]
    window = set(xmas[:25])
    left = 0

    for num in xmas[25:]:
        if not _sum_pair_exists(window, num):
            return num

        window.remove(xmas[left])
        window.add(num)
        left += 1


def _star2(invalid_num: int):
    xmas = [int(line.strip()) for line in open("../../inputs/day9.txt")]
    left, right = 0, 1
    total = xmas[left] + xmas[right]

    while total != invalid_num:
        right += 1
        total += xmas[right]

        while total > invalid_num and left < (right - 1):
            total -= xmas[left]
            left += 1

    result = xmas[left : right + 1]
    return min(result) + max(result)


def day09():
    star1 = _star1()
    star2 = _star2(star1)

    print("DAY 9\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(star1, star2))
