from collections import defaultdict


def _star1() -> int:
    nums = sorted([int(line.strip()) for line in open("../../inputs/day10.txt")])
    num_one_jolts, num_three_jolts = 1, 1

    for i, n in enumerate(nums[:-1]):
        if (nums[i] + 1) == nums[i + 1]:
            num_one_jolts += 1
        if (nums[i] + 3) == nums[i + 1]:
            num_three_jolts += 1

    return num_one_jolts * num_three_jolts


def _star2() -> int:
    nums = [0] + sorted([int(line.strip()) for line in open("../../inputs/day10.txt")])
    counts = defaultdict(int, {0: 1})
    for i in nums[1:]:
        counts[i] = counts[i - 1] + counts[i - 2] + counts[i - 3]
    return counts[nums[-1]]


def day10():
    print("DAY 10\n=====\nSTAR 1: {}\nSTAR 2: {}\n".format(_star1(), _star2()))
