def star1():
    s = set()

    for line in [int(x) for x in open('../inputs/day1.txt', 'r')]:
        if (2020 - line) in s:
            return line * (2020 - line)
        s.add(line)


def star2():
    nums = sorted([int(x) for x in open('../inputs/day1.txt', 'r')])

    for i in range(len(nums) - 2):
        left, right = i+1, len(nums) - 1

        while left < right:
            s = nums[i] + nums[left] + nums[right]
            if s > 2020:
                right -= 1
            elif s < 2020:
                left += 1
            elif s == 2020:
                return nums[i] * nums[left] * nums[right]


print('STAR 1: {}\nSTAR 2: {}'.format(star1(), star2()))
